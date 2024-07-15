// TODO: fix the `assert_eq` at the end of the tests.
//  Do you understand why that's the resulting output?
// テストの最後にある`assert_eq`を修正してください。
// なぜその結果になるか理解していますか？
//
// cspell: disable
// 文字列を半分の長さに分割して、前半の文字列のUTF8バイトシーケンスを送信して、40ミリ秒後(timeout * 2)に、後半の文字列を送信している。
// しかし、`run`関数は、タイムアウトを20ミリ秒に設定して、読み込み操作をしているため、前半の文字列しか読み込めない。
// `tokio::net::TcpStream::read_to_end()`は、読み込んだデータをバッファに追加書き込みする。
// よって、`"hello"`の`"he"`、`"from"`の`"fr"`、`"this"`の`"th"`、`"task"`の`"ta"`が`buffer`に追加書き込みされ、
// `"hefrthta"`になる。
// cspell: enable
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener, n_messages: usize, timeout: Duration) -> Vec<u8> {
    let mut buffer = Vec::new();
    for _ in 0..n_messages {
        let (mut stream, _) = listener.accept().await.unwrap();
        let _ = tokio::time::timeout(timeout, async {
            stream.read_to_end(&mut buffer).await.unwrap();
        })
        .await;
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let messages = vec!["hello", "from", "this", "task"];
        let timeout = Duration::from_millis(20);
        let handle = tokio::spawn(run(listener, messages.len(), timeout));

        for message in messages {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (_, mut writer) = socket.split();

            let (beginning, end) = message.split_at(message.len() / 2);

            // Send first half
            writer.write_all(beginning.as_bytes()).await.unwrap();
            tokio::time::sleep(timeout * 2).await;
            writer.write_all(end.as_bytes()).await.unwrap();

            // Close the write side of the socket
            let _ = writer.shutdown().await;
        }

        let buffered = handle.await.unwrap();
        let buffered = std::str::from_utf8(&buffered).unwrap();
        // cspell: disable-next-line
        assert_eq!(buffered, "hefrthta");
    }
}
