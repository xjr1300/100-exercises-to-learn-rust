// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply clients by sending
//  the `Display` representation of the `reply` argument as a response.
// `fixed_reply`関数を実装してください。
// それは2つの`TcpListener`インスタンスを受け取り、同時並行でそれら両方の接続を受け付け、常に`replay`引数の
// `Display`表現を応答として送信することで、クライアントに応答します。
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

async fn reply_func<T>(listener: TcpListener, reply: Arc<T>)
where
    T: Display + Send + Sync + 'static,
{
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let (_, mut writer) = stream.split();
        let response = format!("{reply}");
        let _ = writer.write_all(response.as_bytes()).await;
    }
}

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    // `T`はクローンされません。どのように2つのサーバータスク間でそれを共有するのでしょうか？
    T: Display + Send + Sync + 'static,
{
    let reply = Arc::new(reply);
    let cloned_reply = Arc::clone(&reply);

    let first_handle = tokio::spawn(reply_func(first, reply));
    let second_handle = tokio::spawn(reply_func(second, cloned_reply));

    let _ = tokio::join!(first_handle, second_handle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
