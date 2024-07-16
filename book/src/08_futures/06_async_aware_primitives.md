# Async-aware primitives（非同期を知っている構成要素）

If you browse `tokio`'s documentation, you'll notice that it provides a lot of types
that "mirror" the ones in the standard library, but with an asynchronous twist:
locks, channels, timers, and more.

> `tokio`ドキュメントを参照した場合、標準ライブラリのモノを「反映した」多くの方が提供されていることに気付くでしょうが、非同期になっています。ロック、チャネル、タイマー、その他。

When working in an asynchronous context, you should prefer these asynchronous alternatives
to their synchronous counterparts.

> 非同期文脈で作業するとき、それらの同期的な同等物ではなく、これら非同期の代替物を優先するべきです。

To understand why, let's take a look at `Mutex`, the mutually exclusive lock we explored
in the previous chapter.

> 理由を理解するために、前の章で探求した可変排他ロックである`Mutex`を確認しましょう。

## Case study: `Mutex`（事例研究: Mutex）

Let's look at a simple example:

> 単純な例を確認してください。

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
    // `guard`はここでドロップされます。
}

/// Use `v` as the body of an HTTP call.
/// HTTP呼び出しの本体として`v`を使用します。
async fn http_call(v: &[u64]) {
  // [...]
}
```

### `std::sync::MutexGuard` and yield points（std::sync::MutexGuardとyieldポイント）

This code will compile, but it's dangerous.

> このコードはコンパイルされますが、危険です。

We try to acquire a lock over a `Mutex` from `std` in an asynchronous context.
We then hold on to the resulting `MutexGuard` across a yield point (the `.await` on
`http_call`).

> `std`の非同期文脈からきた`Mutex`を介してロックを取得することを試みます。
> そして、`http_call`を`.await`している`yield`ポイントをまたいで結果である`MutexGuard`を保持します。

Let's imagine that there are two tasks executing `run`, concurrently, on a single-threaded
runtime. We observe the following sequence of scheduling events:

> 単一スレッドなランタイムで、同時並行で`run`を実行する2つのタスクがある場合を想像してください。
> イベントのスケジューリングの次のＣ館すを観察します。

```text
     Task A          Task B
        |
  Acquire lock
Yields to runtime
        |
        +--------------+
                       |
             Tries to acquire lock
```

We have a deadlock. Task B we'll never manage to acquire the lock, because the lock
is currently held by task A, which has yielded to the runtime before releasing the
lock and won't be scheduled again because the runtime cannot preempt task B.

> デッドロックが発生します。
> タスクBは決してロックを取得できません。
> 現在、ロックはタスクAによって保持されており、タスクAはロックを開放する前にランタイムに移譲しますが、単ライムはタスクBを実行できないため、再度スケジュールされることはありません。

> タスクBがロックを取得しようとしても、タスクAがロックを取得しているため、タスクBは`let guard = m.lock().unwrap();`でブロックされる。

### `tokio::sync::Mutex`

You can solve the issue by switching to `tokio::sync::Mutex`:

> `tokio::sync::Mutex`に切り替えることで、その問題を解決できます。

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
}
```

Acquiring the lock is now an asynchronous operation, which yields back to the runtime
if it can't make progress.\
Going back to the previous scenario, the following would happen:

> 現在、ロックの取得は非同期操作で、それが進めることができない場合、（制御を）ランタイムに移譲します。
> 前のシナリオに戻ると、次が発生します。

```text
       Task A          Task B
          |
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          |
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```

All good!

> すべてがうまくいきます！

### Multithreaded won't save you（マルチスレッドは救いません）

We've used a single-threaded runtime as the execution context in our
previous example, but the same risk persists even when using a multithreaded
runtime.\
The only difference is in the number of concurrent tasks required to create the deadlock:
in a single-threaded runtime, 2 are enough; in a multithreaded runtime, we
would need `N+1` tasks, where `N` is the number of runtime threads.

> 前の例では実行文脈として単一スレッドランタイムを使用しましたが、同じリスクはマルチスレッドランタイムを使用したときも残ります。
> 唯一の違いは、デッドロックを作成するために必要な同時並行タスクの数です。
> 単一スレッドランタイムでは2つで十分でした。
> マルチスレッドランタイムでは、`N+1`のタスクが必要で、`N`はランタイムスレッドの数です。

### Downsides（欠点）

Having an async-aware `Mutex` comes with a performance penalty.\
If you're confident that the lock isn't under significant contention
_and_ you're careful to never hold it across a yield point, you can
still use `std::sync::Mutex` in an asynchronous context.

> 非同期を理解している`Mutex`を使用することは、パフォーマンスの不利益を伴います。
> ロックが競合状態でないことに自身があり、`yield`ポイントをまたいでロックを決して保持しないように注意している場合、非同期文脈で、まだ`std::sync::Mutex`を使用できます。

But weigh the performance benefit against the liveness risk you
will incur.

> ただし、パフォーマンス上の利点と、発生する非生存リスクを比較検討してください。

## Other primitives（ほかの構成要素）

We used `Mutex` as an example, but the same applies to `RwLock`, semaphores, etc.\
Prefer async-aware versions when working in an asynchronous context to minimise
the risk of issues.

> 例として`Mutex`を使用しましたが、`RwLock`、セマフォなどにも同じことが適用されます。
> 問題のリスクを最小化するために非同期文脈でわ行するときは、非同期を理解しているバージョンを優先してください。
