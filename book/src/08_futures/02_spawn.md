# Spawning tasks（タスクを生み出す）

Your solution to the previous exercise should look something like this:

> 前の演習の解答は次のようになるはずです。

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

This is not bad!\
If a long time passes between two incoming connections, the `echo` function will be idle
(since `TcpListener::accept` is an asynchronous function), thus allowing the executor
to run other tasks in the meantime.

> それは悪くありません！
> 2つの着信接続の間で長い時間が経過すると、`TcpListener::accept`が非同期関数であるため、`echo`関数はアイドル状態になり、その間、エグゼキューターに他のタスクを実行させます。

But how can we actually have multiple tasks running concurrently?\
If we always run our asynchronous functions until completion (by using `.await`), we'll never
have more than one task running at a time.

> しかし、実際、どのように複数のタスクを同時並行で実行するのでしょうか？
> `.await`を使用して、常に完了するまで非同期関数を実行した場合、一度に1つのタスクしか実行できません。

This is where the `tokio::spawn` function comes in.

> これが、`tokio::spawn`関数が登場します。

## `tokio::spawn`

`tokio::spawn` allows you to hand off a task to the executor, **without waiting for it to complete**.\
Whenever you invoke `tokio::spawn`, you're telling `tokio` to continue running
the spawned task, in the background, **concurrently** with the task that spawned it.

> `tokio::spawn`は、**タスクが完了することを待たずに**タスクをエグゼキューターに渡します。
> `tokio::spawn`を呼び出すときはいつでも、生成されたタスクとそのタスクを生成したタスクを、**同時並行**でバックグラウンドで実行を継続するように`tokio`に伝えます。

Here's how you can use it to process multiple connections concurrently:

> 複数の接続を同時並行で処理するために、それ（`tokio::spawn`）を使用する方法を次に示します。

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        // `listener.accept().await?`により、`echo`関数は新しい接続を受け付けるまで、ここで待機する。
        // よって、この`loop`は、新しい接続が到着するたびに再開される。
        // リスナーの接続が切れた場合、`listener.accept().await?`はエラーを返すため、`echo`関数は終了する。
        let (mut socket, _) = listener.accept().await?;
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start
        // accepting new connections
        // メインタスクにすぐに新しい接続の受付を開始させるために、接続を処理するバックグラウンドタスクを生み出します。
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### Asynchronous blocks（非同期ブロック）

In this example, we've passed an **asynchronous block** to `tokio::spawn`: `async move { /* */ }`
Asynchronous blocks are a quick way to mark a region of code as asynchronous without having
to define a separate async function.

> この例において、`tokio::spawn`に`async move { /* */ }`の**非同期ブロック**を渡しました。
> 非同期ブロックは、分離した非同期関数を定義する必要なく、非同期としてコード領域をマークする簡単な方法です。

### `JoinHandle`

`tokio::spawn` returns a `JoinHandle`.\
You can use `JoinHandle` to `.await` the background task, in the same way
we used `join` for spawned threads.

> `tokio::spawn`は`JoinHandle`を返します。
> 生み出したスレッドに対して`join`を使用した同じ方法で、バックグラウンドタスクを`.await`するために`JoinHandle`を使用できます。

```rust
// 測定データをリモートサーバーに運搬して、その間に、自分の他の仕事をする。
// 自分の他の仕事が完了したら、リモートサーバーに測定データが運搬されるまで待機する。
pub async fn run() {
    // Spawn a background task to ship telemetry data
    // to a remote server
    // 測定データをリモートサーバーに運搬するために、バックグラウンドタスクを生み出します。
    let handle = tokio::spawn(emit_telemetry());
    // In the meantime, do some other useful work
    // その間、任意の他の有益な仕事をします。
    do_work().await;
    // But don't return to the caller until
    // the telemetry data has been successfully delivered
    // しかし、測定データが成功裏に配送されるまで、呼び出し側に戻らないでください。
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### Panic boundary（パニック境界）

If a task spawned with `tokio::spawn` panics, the panic will be caught by the executor.\
If you don't `.await` the corresponding `JoinHandle`, the panic won't be propagated to the spawner.
Even if you do `.await` the `JoinHandle`, the panic won't be propagated automatically.
Awaiting a `JoinHandle` returns a `Result`, with [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)
as its error type. You can then check if the task panicked by calling `JoinError::is_panic` and
choose what to do with the panic—either log it, ignore it, or propagate it.

> `tokio::spawn`で生み出されたタスクがパニックした場合、そのパニックはエグゼキューターによって受け取られます。
> 対応する`JoinHandle`を`.await`しない場合、そのパニックは（タスクの）生み出し元には伝播しません。
> `JoinHandle`を`.await`したとしても、そのパニックは自動的に伝播しません。
> `JoinHandle`を`.await`すると、そのエラー型として`JoinError`を持つ`Result`が返されます。
> そして、`JoinError::is_panic`を呼び出すことで、タスクがパニックしたか確認して、それをログに記録するか、またはそれを伝播するか、パニックに対して行うことを選択できます。

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // The task has panicked
            // We resume unwinding the panic,
            // thus propagating it to the current task
            // タスクがパニックしました。
            // パニックの巻き戻しを再開して、現在のタスクにパニックを伝播しています。
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

> `panic`を`unwind`するとは、`panic`が発生したスレッドのスタックフレームを巻き戻して、クリーンアップコードを実行して、リソースを適切に解放する処理を示す。

### `std::thread::spawn` vs `tokio::spawn`（std::thread::spawnとtokio::spawn）

You can think of `tokio::spawn` as the asynchronous sibling of `std::thread::spawn`.

> `tokio::spawn`を`std::thread::spawn`の非同期の兄弟と考えることができます。

Notice a key difference: with `std::thread::spawn`, you're delegating control to the OS scheduler.
You're not in control of how threads are scheduled.

> 重要な違いに注意してください: `std::thread::spawn`を使用すると、OSのスケジューラーに制御を移譲します。
> スレッドがスケジュールされる方法を制御することはできません。

With `tokio::spawn`, you're delegating to an async executor that runs entirely in
user space. The underlying OS scheduler is not involved in the decision of which task
to run next. We're in charge of that decision now, via the executor we chose to use.

> `tokio::spawn`を使用すると、ユーザー空間で全体を実行する非同期エグゼキューターに（制御を）移譲します。
> 基盤となるOSスケジューラーは、次に実行するタスクの決定に関与しません。
> 現在、その決定の責任は、使用することを選択したエグゼキューターを介して、開発者にあります。
