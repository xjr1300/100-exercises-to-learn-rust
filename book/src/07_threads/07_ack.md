# Two-way communication（双方向通信）

In our current client-server implementation, communication flows in one direction: from the client to the server.\
The client has no way of knowing if the server received the message, executed it successfully, or failed.
That's not ideal.

> 現在のクライアントサーバーの実装において、通信の流れはクライアントからサーバーへ向かう一方向です。
> サーバーがメッセージを受け取った場合、正常に実行されたかそれとも失敗したか、クライアントは知る方法がありません。
> これは理想的ではありません。

To solve this issue, we can introduce a two-way communication system.

> この問題を解決するために、双方向通信システムを導入できます。

## Response channel（応答チャネル）

We need a way for the server to send a response back to the client.\
There are various ways to do this, but the simplest option is to include a `Sender` channel in
the message that the client sends to the server. After processing the message, the server can use
this channel to send a response back to the client.

> サーバーがクライアントに応答を送信する方法が必要です。
> これを行う様々な方法がありますが、最も単純な選択肢は、クライアントがサーバーに送信するメッセージ内に`Sender`チャネルを含めることです。
> メッセージを処理した後、サーバーはクライアントにレスポンスを送信するために、このチャネルを使用できます。

This is a fairly common pattern in Rust applications built on top of message-passing primitives.

> これは、メッセージパッシング構成要素の上に構築されたRustアプリケーションの中で、かなり一般的なパターンです。
