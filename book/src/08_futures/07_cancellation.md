# Cancellation（キャンセル）

What happens when a pending future is dropped?\
The runtime will no longer poll it, therefore it won't make any further progress.
In other words, its execution has been **cancelled**.

> 保留中のフューチャーがドロップされたとき何が起こるでしょうか？
> ランタイムをもはやそれをポーリングしないため、それ以上進みません。
> 言い換えれば、その実行は**キャンセル**されます。

In the wild, this often happens when working with timeouts.
For example:

> 実際に、これはタイムアウトを使用したときによく発生します。
> 例えば・・・

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
    // 期限を10ミリ秒に設定した`Timeout`でフューチャーをラップします。
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

When the timeout expires, the future returned by `http_call` will be cancelled.
Let's imagine that this is `http_call`'s body:

> 時間切れになったとき、`http_call`によって返されたフューチャーはキャンセルされます。
> 次が`http_call`の本体であると想像しましょう。

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

Each yield point becomes a **cancellation point**.\
`http_call` can't be preempted by the runtime, so it can only be discarded after
it has yielded control back to the executor via `.await`.
This applies recursively—e.g. `stream.write_all(&request)` is likely to have multiple
yield points in its implementation. It is perfectly possible to see `http_call` pushing
a _partial_ request before being cancelled, thus dropping the connection and never
finishing transmitting the body.

> それぞれの移譲ポイントは、**キャンセルポイント**になります。
> `http_call`はランタイムによってプリエンプトされることはないため、それは`.await`を介してエグゼキューターに制御を移譲した後にのみ、破棄されるます。
> これは、再帰的に適用されます。
> 例えば、`stream.write_all(&request)`は、その実装内に複数の移譲ポイントを持つ可能性が高いです。
> `http_call`がキャンセルされる前に _部分的な_ リクエストを押入れ、接続が切断されて、本体の送信が終了しない可能性が十分にあります。

> **プリエンプト**: プロセスに与えられたCPU時間を消費したときに、そのプロセスの実行を中断して、他のプロセスにCPUを割り当てること。

## Clean up（クリーンアップ）

Rust's cancellation mechanism is quite powerful—it allows the caller to cancel an ongoing task
without needing any form of cooperation from the task itself.\
At the same time, this can be quite dangerous. It may be desirable to perform a
**graceful cancellation**, to ensure that some clean-up tasks are performed
before aborting the operation.

> Rustのキャンセルのメカニズムはとても強力です。
> それは、タスク自身からのいかなる形式の協力を必要としないで、呼び出し側に実行中のタスクをキャンセルできるようにします。
> それと同時に、これはとても危険になる可能性があります。
> それは、操作を中止する前に、いくつかのクリーンアップタスクを実行することを確実にするために、**優雅なキャンセル**を実行することが望ましいかもしれません。

For example, consider this fictional API for a SQL transaction:

> 例えば、SQLトランザクションのために、次の架空のAPIを考えます。

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

On cancellation, it'd be ideal to explicitly abort the pending transaction rather
than leaving it hanging.
Rust, unfortunately, doesn't provide a bullet-proof mechanism for this kind of
**asynchronous** clean up operations.

> キャンセルする場合、保留中のトランザクションを明示的に中止することが、その応答がないまま残しておくよりも理想的です。
> 不運にも、Rustはこの種の**非同期**クリーンアップ操作を行うための強固なメカニズムを提供していません。

The most common strategy is to rely on the `Drop` trait to schedule the required
clean-up work. This can be by:

- Spawning a new task on the runtime
- Enqueueing a message on a channel
- Spawning a background thread

> 最も一般的な戦略は、要求されるクリーンアップ作業をスケジュールするために`Drop`トレイトに依存することです。
> これは、次によって行えます。
>
> - ランタイム上で新しいタスクを生み出します。
> - チャネル上のメッセージをキューに入れます。
> - バックグラウンドスレッドを生み出します。

The optimal choice is contextual.

> 最適な選択肢は、状況によって異なります。

## Cancelling spawned tasks（生み出したタスクのキャンセル）

When you spawn a task using `tokio::spawn`, you can no longer drop it;
it belongs to the runtime.\
Nonetheless, you can use its `JoinHandle` to cancel it if needed:

> `tokio::spawn`を使用してタスクを生み出したとき、もはやそれをドロップできません。
> それはランタイムに属しています。
> それにも関わらず、必要に応じて、それをキャンセルするために、その`JoinHandle`を使用できます。

```rust
async fn run() {
    let handle = tokio::spawn(/* some async task（何らかの非同期タスク） */);
    // Cancel the spawned task
    // 生み出したタスクをキャンセルします。
    handle.abort();
}
```

## Further reading（参考資料）

- Be extremely careful when using `tokio`'s `select!` macro to "race" two different futures.
  Retrying the same task in a loop is dangerous unless you can ensure **cancellation safety**.
  Check out [`select!`'s documentation](https://tokio.rs/tokio/tutorial/select) for more details.\
  If you need to interleave two asynchronous streams of data (e.g. a socket and a channel), prefer using
  [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) instead.
- Rather than "abrupt" cancellation, it can be preferable to rely
  on [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html).

> - `tokio`の`select!`マクロを使用して2つの異なるフューチャーを「競合」させる場合は、細心の注意を払ってください。
>   **キャンセルの安全性**を確保できない限り、ループ内で同じタスクを再試行することは危険です。
>   詳細は`select!`のドキュメントを確認してください。
>   例えばソケットとチャネルなど、2つの非同期ストリームのデータを交互に処理する必要がある場合、代わりに`StreamExt::merge`を使用することを優先してください。
> - 「突然の」キャンセルよりも、`CancellationToken`に依存することが好ましい場合があります。
