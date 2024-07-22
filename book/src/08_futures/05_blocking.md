# Don't block the runtime（ランタイムをブロックしてはならない）

Let's circle back to yield points.\
Unlike threads, **Rust tasks cannot be preempted**.

> `移譲`ポイントにぐるっと戻りましょう。
> スレッドと異なり、**Rustのタスクは、プリエンプトできません**。

> プリエンプト: システムのスケジューリングにおいて、システムの応答性や効率性を高めるために、あるプロセスやタスクがCPUの制御を強制的に中断され、他のプロセスやタスクに制御が移されること。

`tokio` cannot, on its own, decide to pause a task and run another one in its place.
The control goes back to the executor **exclusively** when the task yields—i.e.
when `Future::poll` returns `Poll::Pending` or, in the case of `async fn`, when
you `.await` a future.

> `tokio`自身は、タスクを停止して、その場所で他のタスクを実行することを決定できません。
> その制御は、タスクが移譲するとき、**排他的に**エグゼキューターに戻ります。
> 例えば、`Future::poll`が`Poll::Pending`を返したとき、または`async fn`の場合はフューチャーを`.await`したときです。

This exposes the runtime to a risk: if a task never yields, the runtime will never
be able to run another task. This is called **blocking the runtime**.

> これは、ランタイムがリスクをさらけ出すことになります。
> タスクが決して移譲されない場合、ランタイムは決して他のタスクを実行できません。
> これは**ランタイムのブロッキング**と呼ばれます。

## What is blocking?（ブロッキングとは何か？）

How long is too long? How much time can a task spend without yielding before it
becomes a problem?

> どれくらいが長いのでしょうか？
> 問題になるまで、移譲しないタスクはどれくらいの時間を費やすことができるでしょうか？

It depends on the runtime, the application, the number of in-flight tasks, and
many other factors. But, as a general rule of thumb, try to spend less than 100
microseconds between yield points.

> それはランタイム、アプリケーション、実行中のタスクの数、そして多くの他の要因に依存します。
> しかし、一般的な経験則として、移譲ポイント間で100マイクロ秒より少ない時間を費やすようにしてください。

## Consequences（結果）

Blocking the runtime can lead to:

- **Deadlocks**: if the task that's not yielding is waiting for another task to
  complete, and that task is waiting for the first one to yield, you have a deadlock.
  No progress can be made, unless the runtime is able to schedule the other task on
  a different thread.
- **Starvation**: other tasks might not be able to run, or might run after a long
  delay, which can lead to poor performances (e.g. high tail latencies).

> ランタイムのブロックは次を引き起こす可能性があります。
>
> - **デッドロック**: 移譲していないタスクが他のタスクが完了することを待っていて、またそのタスク（他のタスク）が最初のタスク（移譲していないタスク）が移譲するのを待っている場合、デッドロックになります。
>   ランタイムは異なるスレッドに他のタスクをスケジュールできるにも関わらず、何も進みません。
> - **飢餓**: 他のタスクは実行できないか、長い遅延の後に実行されるかもしれない場合、それは性能の悪化を引き起こす可能性があります。例えばテールレイテンシーです。

## Blocking is not always obvious（ブロッキングは常に明らかでない）

Some types of operations should generally be avoided in async code, like:

- Synchronous I/O. You can't predict how long it will take, and it's likely to be
  longer than 100 microseconds.
- Expensive CPU-bound computations.

> いくつかの種類の操作は、一般的に非同期コードでは避けられるべきです。次のように・・・
>
> - 同期I/O。それにかかる時間を予測できず、それは100マイクロ秒よりも長くなるかもしれません。
> - 高いCPUバウンドの計算。

The latter category is not always obvious though. For example, sorting a vector with
a few elements is not a problem; that evaluation changes if the vector has billions
of entries.

> ただし、後者のカテゴリーは常に明らかではありません。
> 例えば、いくつかの要素を持つベクターをソートすることは問題になりません。
> ベクターが数十億のエントリを持つ場合、評価は変わります。

## How to avoid blocking（ブロッキングを避ける方法）

OK, so how do you avoid blocking the runtime assuming you _must_ perform an operation
that qualifies or risks qualifying as blocking?\
You need to move the work to a different thread. You don't want to use the so-called
runtime threads, the ones used by `tokio` to run tasks.

> では、ブロッキングとして認識される、または認識されるリスクがある操作を実行 _しなければならない_ 場合、どのようにランタイムのブロッキングを避けたらよいでしょうか？
> 異なるスレッドにその作業を移動する必要があります。
> ランタイムスレッドと呼ばれるスレッドを使用しないでください。それは`tokio`がタスクを実行するために使用されます。

`tokio` provides a dedicated threadpool for this purpose, called the **blocking pool**.
You can spawn a synchronous operation on the blocking pool using the
`tokio::task::spawn_blocking` function. `spawn_blocking` returns a future that resolves
to the result of the operation when it completes.

> `tokio`は、この目的のために**ブロッキングプール**と呼ばれる専用のスレッドプールを提供しています。
> `tokio::task::spawn_blocking`関数を使用して、ブロッキングプール上で同期操作を生み出すことができます。
> `spawn_blocking`は、それが完了したとき、操作の結果を解決するフューチャーを返します。

```rust
use tokio::task;

fn expensive_computation() -> u64 {
    // [...]
}

async fn run() {
    let handle = task::spawn_blocking(expensive_computation);

    // Do other stuff in the meantime
    // この間（上記の高コストな関数を実行している間）に他のことをします。

    // `spawn_blocking`はフューチャーを返すため、`await`して高コストな関数の結果が得られるまで`.await`します。
    let result = handle.await.unwrap();
}
```

The blocking pool is long-lived. `spawn_blocking` should be faster
than creating a new thread directly via `std::thread::spawn`
because the cost of thread initialization is amortized over multiple calls.

> ブロッキングプールは長生きです。
> `spawn_blocking`は、`std::thread::spawn`を介して直接新しいスレッドを作成するよりも早いはずです。
> それは、スレッドの初期化のコストが、複数の呼び出しにわたって分割されるからです。

## Further reading（参考資料）

- Check out [Alice Ryhl's blog post](https://ryhl.io/blog/async-what-is-blocking/)
  on the topic.
