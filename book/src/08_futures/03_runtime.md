# Runtime architecture（ライフタイムアーキテクチャー）

So far we've been talking about async runtimes as an abstract concept.
Let's dig a bit deeper into the way they are implemented—as you'll see soon enough,
it has an impact on our code.

> これまで、非同期ランタイムについて抽象化概念として話してきました。
> それらが実装された方法を少し深堀りしましょう。すぐに十分理解できるように、それはコードに影響を与えます。

## Flavors（風味づけ）

`tokio` ships two different runtime _flavors_.

> `tokio`は2つの異なるランタイム _フレーバー_ を提供しています。

You can configure your runtime via `tokio::runtime::Builder`:

- `Builder::new_multi_thread` gives you a **multithreaded `tokio` runtime**
- `Builder::new_current_thread` will instead rely on the **current thread** for execution.

> `tokio::runtime::Builder`を介してランタイムを構成できます。
>
> - `Builder::new_multi_thread`は、**マルチスレッドな`tokio`ランタイム**を提供します。
> - `Builder::new_current_thread`は、代わりに**現在のスレッド**に依存して実行されます。

`#[tokio::main]` returns a multithreaded runtime by default, while
`#[tokio::test]` uses a current thread runtime out of the box.

> `#[tokio::main]`は、デフォルトでマルチスレッドなランタイムを返す一方で、`#[tokio::test]`は現在のスレッドを使用します。

### Current thread runtime（カレントスレッドランタイム）

The current-thread runtime, as the name implies, relies exclusively on the OS thread
it was launched on to schedule and execute tasks.\
When using the current-thread runtime, you have **concurrency** but no **parallelism**:
asynchronous tasks will be interleaved, but there will always be at most one task running
at any given time.

> 名前が暗に意味する通り、カレントスレッドランタイムは、タスクをスケジュールして実行するために起動されるOSスレッドのみに依存します。
> カレントスレッドランタイムを使用したとき、**同時並行性**を得られますが、**並列性**は得られません。
> 非同期タスクは、間をおいて交互に実行されますが、常に特定の時間に最大1つのタスクしが実行していません。

### Multithreaded runtime（マルチスレッドランタイム）

When using the multithreaded runtime, instead, there can up to `N` tasks running
_in parallel_ at any given time, where `N` is the number of threads used by the
runtime. By default, `N` matches the number of available CPU cores.

> マルチスレッドランタイムを使用したとき、特定の時間に _並列で_ 最大`N`個までのタスクを実行でき、`N`はランタイムで使用されるスレッドの数です。
> デフォルトで、`N`は利用可能なCPUコアの数に一致します。

There's more: `tokio` performs **work-stealing**.\
If a thread is idle, it won't wait around: it'll try to find a new task that's ready for
execution, either from a global queue or by stealing it from the local queue of another
thread.\
Work-stealing can have significant performance benefits, especially on tail latencies,
whenever your application is dealing with workloads that are not perfectly balanced
across threads.

> まだあります。`tokio`は**ワークスチーリング**を行います。
> スレッドがアイドル状態になった場合、それ（`tokio`）は待機しません。
> それ（`tokio`）は、グローバルキューまたは他のスレッドのローカルキューから盗むことで、実行の準備ができている新しいタスクを見つけることを試みます。
> アプリケーションが、スレッド間で完全にバランスが取れていないワークロードを処理しているときはいつでも、ワークスチーリングは、特にテールレイテンシーに対して、大きな性能の利点をもたらす可能性があります。

> テールレイテンシー（`tail latency`）とは、システムにおいて最も遅いリクエストの応答時間を指す。
> 具体的には、応答時間の分布における上位のパーセンタイル、例えば95パーセンタイルや99パーセンタイルの応答時間を示す。
> 応答時間の長いタスクがある場合、その長いタスクがアイドル状態になったときに、他のタスクを実行することで、全体の応答時間を短くすることができる。

## Implications（関連事項）

`tokio::spawn` is flavor-agnostic: it'll work no matter if you're running on the multithreaded
or current-thread runtime. The downside is that the signature assume the worst case
(i.e. multithreaded) and is constrained accordingly:

> `tokio::spawn`はフレーバーに依存しません。
> それは、マルチスレッドまたはカレントスレッドランタイムで実行されているかどうかに関わらず機能します。
> その欠点は、シグネチャーが最悪のケース（つまりマルチスレッド）を仮定して、それに応じて制約されることです。

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{ /* */ }
```

Let's ignore the `Future` trait for now to focus on the rest.\
`spawn` is asking all its inputs to be `Send` and have a `'static` lifetime.

> 現時点では、`Future`トレイトを無視して、残りに焦点を当てましょう。
> `spawn`は、その入力に対して、`Send`であり、`'static`ライフタイムを持つことを要求しています。

The `'static` constraint follows the same rationale of the `'static` constraint
on `std::thread::spawn`: the spawned task may outlive the context it was spawned
from, therefore it shouldn't depend on any local data that may be de-allocated
after the spawning context is destroyed.

> `'static`制約は、`std::thread::spawn`の`'static`制約と同じ論理的根拠に従っています。
> 生み出されたタスクは、それが生み出されたコンテキストよりも長生きするかもしれず、そのため、それは生み出したコンテキストが破壊された後で、解放されてしまうかもしれない任意のローカルデータに依存するべきではありません。

```rust
fn spawner() {
    let v = vec![1, 2, 3];
    // This won't work, since `&v` doesn't
    // live long enough.
    // `&v`は十分に長生きしないため、これは機能しません。
    tokio::spawn(async {
        for x in &v {
            println!("{x}")
        }
    })
    // `spawn`したタスクが終了する前に、`spawner`関数が終了して、
    // `v`が解放される可能性があるため、Rustはこのコードをコンパイルさせない。
}
```

`Send`, on the other hand, is a direct consequence of `tokio`'s work-stealing strategy:
a task that was spawned on thread `A` may end up being moved to thread `B` if that's idle,
thus requiring a `Send` bound since we're crossing thread boundaries.

> 一方、`Send`は`tokio`のワークスチール戦略の直接な結果です。
> スレッド`A`で生み出されたタスクは、それがアイドル状態になった場合、最終的にスレッド`B`にムーブされるかもしれず、そのためスレッド境界を超えるために`Send`境界が要求されます。

> スレッド`A`が生み出されたタスクが、IO待ちなどでブロックされた場合、そのタスクはキューに配置される。
> スレッド`A`が他のタスクを実行している間、キューに入ったタスクが要求したIO処理が完了して、タスクを実行できる状態になる。
> このとき、スレッド`A`がまだ他のタスクを実行していて、スレッド`B`が実行していたタスクがブロックまたは終了した場合、
> スレッド`B`は、スレッド`A`のキューに入ったタスクを**盗んで**実行する。
> ただし、これはスレッド自身が行うのではなく、ランタイムのエグゼキューターがタスクの割り当てを行う。

```rust
fn spawner(input: Rc<u64>) {
    // This won't work either, because
    // `Rc` isn't `Send`.
    // `Rc`は`Send`でないため、これも機能しません。
    tokio::spawn(async move {
        println!("{}", input);
    })
}
```

> `Rc`はそのスレッド間で共有できない。
> `Rc`が持つ参照カウンターは`usize`型でアトミックに参照カウンターを増減できない。
>
> ```rust
> #[repr(C)]
> struct RcBox<T: ?Sized> {
>     strong: Cell<usize>,
>     weak: Cell<usize>,
>     value: T,
> }
> ```
>
> もし、`Rc`が`Send`な場合、クローンして別のスレッドに渡すことができ、このときそれぞれのスレッドで参照カウンターを増減することになる。
> その場合、参照カウンターはアトミックな操作ができないため、参照カウンターは正しい値を保持できない。
> よって、`Rc`は`Send`でない。
