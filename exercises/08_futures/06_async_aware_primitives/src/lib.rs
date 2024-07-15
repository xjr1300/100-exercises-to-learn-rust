/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
/// 非同期を理解していない標準ライブラリのチャネルを使用しているため、下のコードをはデッドロックします。
/// `tokio`のチャネル構成要素を使用してそれを再記述してください。
/// テストコードにも触れる必要があります。
///
/// Can you understand the sequence of events that can lead to a deadlock?
/// デッドロックを引き起こす可能性があるイベントのシーケンスを理解できますか？
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
/// 受け取ったメッセージに`pong`で応答して、呼び出し側と会話を継続するために新しいチャネルを準備してください。
pub async fn pong(mut receiver: mpsc::Receiver<Message>, buffer_size: usize) {
    loop {
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(buffer_size);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let buffer_size = 128;
        let (sender, receiver) = mpsc::channel(buffer_size);
        let (response_sender, mut response_receiver) = mpsc::channel(buffer_size);

        let result = sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await;
        assert!(result.is_ok());

        tokio::spawn(pong(receiver, buffer_size));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}

// # `std::sync::mpsc::Receiver::recv()`
//
// この受信者に送信された値を待つことを試みて、対応するチャネルがハングアップした場合エラーを返す。
// この関数は、利用できるデータがない場合や、少なくとも1つ以上の送信者がまだ存在して、データが送信される可能性がある場合、常にカレントスレッドをブロックする。
// 一度、メッセージが対応する`Sender`または`SyncSender`に送信されると、この受信者は目覚めてメッセージを送信する。
// 対応する`Sender`が切断した場合、またはこの呼び出しがブロックしている間に切断した場合、この呼び出しは目覚めさせて、このチャネルで受診できるメッセージがないことを
// 示す`Err`を返す。
// しかし、チャネルがバッファーされた場合、切断される前に送信されたメッセージは、適切に受信される。

// 標準ライブラリのmpscがデッドロックする理由は次の通り。
//
// `ping`は、`pong`側にメッセージを送信して、`pong`を別スレッドで起動した後で、次の通り`pong`からのレスポンスを待つ。
//
// ```rust
// let answer = response_receiver.recv().unwrap().payload;
// ```
//
// しかし、`recv`は現在のスレッドをブロックして、`pong`に制御が渡らず、そのため`pong`は`ping`にレスポンスを送信できない。
// よって、`pong`からのレスポンスが得られないため、上記でデッドロックする。
// この状況は、デバッグプリントで確認できる。
//
// ```text
// before sending in ping
// after sending in ping
// before spawning pong
// after spawning pong
// before receiving in ping
// ```
//
// 例えば、`pong`を別スレッドで起動した後にスリープすると、`pong`の処理は進むが、`recv()`で停止する。
// それは、`pong`が`ping`に送信しようとする`msg.response_channel`チャネルが、`ping`と同じスレッドで生成されており、
// そのスレッドは`recv()`でブロックされているから、送信処理が完了せずにデッドロックしたままである。
//
// ```rust
// tokio::spawn(pong(receiver));
//
// tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//
// let answer = response_receiver.recv().unwrap().payload;
// ```
//
// ```text
// before sending in ping
// after sending in ping
// before spawning pong
// after spawning pong
// before looping in pong
// before receiving in pong
// Pong received: pong
// after sending response in pong
// before receiving in pong
// ```
//
// これを`tokio`に変更すると、処理が進められなくなった場合、処理を進めようとしたスレッドをブロックする代わりに、
// 他のタスクに制御を移譲するため、処理が進むようになる。
// これは、スレッドの同期並行処理（concurrent）と、`tokio`の非同期並列処理（parallel）の違いである。
/*
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    println!("before looping in pong");
    loop {
        println!("before receiving in pong");
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            println!("after sending response in pong");
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();

        println!("before sending in ping");
        let result = sender.send(Message {
            payload: "pong".into(),
            response_channel: response_sender,
        });
        assert!(result.is_ok());
        println!("after sending in ping");

        println!("before spawning pong");
        tokio::spawn(pong(receiver));
        println!("after spawning pong");

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        println!("before receiving in ping");
        let answer = response_receiver.recv().unwrap().payload;
        println!("after receiving in ping");
        assert_eq!(answer, "pong");
    }
}
*/
