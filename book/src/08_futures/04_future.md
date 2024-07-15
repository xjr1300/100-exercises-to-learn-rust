# The `Future` trait（Futureトレイト）

## The local `Rc` problem （ローカルRc問題）

Let's go back to `tokio::spawn`'s signature:

> `tokio::spawn`のシグネチャーに戻りましょう。

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

What does it _actually_ mean for `F` to be `Send`?\
It implies, as we saw in the previous section, that whatever value it captures from the
spawning environment has to be `Send`. But it goes further than that.

> `F`が`Send`であるとは_実際に_何を意味しているのでしょうか？
> それは、前のセクションで確認した通り、生み出した環境からキャプチャーされた値が`Send`でなければならないことを暗示しています。
> しかし、それはそれ以上のことを意味しています。

Any value that's _held across a .await point_ has to be `Send`.\
Let's look at an example:

> _`.await`ポイントをまたがって保持される_ 任意の値は、`Send`でなければなりません。
> 例を確認しましょう。

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // A value that's not `Send`,
    // created _inside_ the async function
    // 非同期関数内で作成されたその値は`Send`ではありません。
    let non_send = Rc::new(1);

    // A `.await` point that does nothing
    // 何もしない`.await`ポイントです。
    yield_now().await;

    // The local non-`Send` value is still needed
    // after the `.await`
    // `.await`の後も、ローカルな非`Send`の値が必要です。
    println!("{}", non_send);
}
```

The compiler will reject this code:

> コンパイラーはこのコードを拒否します。

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

To understand why that's the case, we need to refine our understanding of
Rust's asynchronous model.

> そのケースを理由を理解するために、Rustの非同期モデルの理解を洗練させる必要があります。

## The `Future` trait（Futureトレイト）

We stated early on that `async` functions return **futures**, types that implement
the `Future` trait. You can think of a future as a **state machine**.
It's in one of two states:

- **pending**: the computation has not finished yet.
- **ready**: the computation has finished, here's the output.

> `async`関数が、`Future`トレイトを実装した型である**フューチャー**を返すと述べました。
> フューチャーを**状態マシン**と考えることができます。
> それは２つの状態を持ちます。
>
> - **保留中**: 計算はまだ終了していません。
> - **準備完了**: 計算が終了して、ここに出力があります。

This is encoded in the trait definition:

> これは、トレイト定義内にエンコードされています。

```rust
trait Future {
    type Output;

    // Ignore `Pin` and `Context` for now
    // 現時点では`Pin`と`Context`を無視してください。
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### `poll`

The `poll` method is the heart of the `Future` trait.\
A future on its own doesn't do anything. It needs to be **polled** to make progress.\
When you call `poll`, you're asking the future to do some work.
`poll` tries to make progress, and then returns one of the following:

- `Poll::Pending`: the future is not ready yet. You need to call `poll` again later.
- `Poll::Ready(value)`: the future has finished. `value` is the result of the computation,
  of type `Self::Output`.

> `poll`メソッドは、`Future`トレイドの心臓です。
> フューチャーそれ自身は、何もしません。それはするメルために**ポーリング**される必要があります。
> `poll`を呼び出したとき、何か作業を行うことをフューチャーに要求します。
> `poll`は、進めようと試み、そして、次の1つを返します。
>
> - `Poll::Pending`: フューチャーはまだ準備できていません。後で再度`poll`を呼び出す必要があります。
> - `Poll::Ready(value)`: フューチャーは完了しました。`value`は計算の結果で、型は`Self::Output`です。

Once `Future::poll` returns `Poll::Ready`, it should not be polled again: the future has
completed, there's nothing left to do.

> 一旦、`Future::poll`が`Poll::Ready`を返すと、それは再度ポーリングされるべきではありません。そのフューチャーは完了しており、行うことは何も残っていません。

### The role of the runtime（ランタイムの役目）

You'll rarely, if ever, be calling poll directly.\
That's the job of your async runtime: it has all the required information (the `Context`
in `poll`'s signature) to ensure that your futures are making progress whenever they can.

> ほとんど、または決して、直接ポーリングすることはありません。
> それは、非同期ランタイムの仕事です。それは、フューチャーは可能なときはいつでも、フューチャーを進めることを確実にするために、`poll`のシグネチャー内の`Context`として、必要な情報をすべて持っています。

## `async fn` and futures（async fnとフューチャー）

We've worked with the high-level interface, asynchronous functions.\
We've now looked at the low-level primitive, the `Future trait`.

> 非同期関数の高水準なインターフェイスで作業をしてきました。
> ここでは、`Future`トレイトの低水準な構成要素を確認しました。

How are they related?

> それらはどのように関連しているのでしょうか？

Every time you mark a function as asynchronous, that function will return a future.
The compiler will transform the body of your asynchronous function into a **state machine**:
one state for each `.await` point.

> 非同期関数を作成するたびに、その関数はフューチャーを返します。
> コンパイラーは、それぞれの`.await`ポイントに1つの状態があるような**状態マシン**に、非同期関数の本体を変換します。

Going back to our `Rc` example:

> `Rc`の例に戻りましょう。

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

The compiler would transform it into an enum that looks somewhat like this:

> コンパイラーは、それを次のように見える列挙型に変換します。

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>), // Rcを保持している。RcはSendでない。よってexampleはSendでない。
    Terminated,
}
```

When `example` is called, it returns `ExampleFuture::NotStarted`. The future has never
been polled yet, so nothing has happened.\
When the runtime polls it the first time, `ExampleFuture` will advance until the next
`.await` point: it'll stop at the `ExampleFuture::YieldNow(Rc<i32>)` stage of the state
machine, returning `Poll::Pending`.\
When it's polled again, it'll execute the remaining code (`println!`) and
return `Poll::Ready(())`.

> `example`が呼び出されたとき、それは`ExampleFuture::NotStarted`を返します。
> フューチャーは、まだ決してポーリングされていないため、何も発生していません。
> ランタイムが最初にそれをポーリングしたとき、`ExampleFuture`は次の`.await`ポイントまで進めます。
> それは、状態マシンの`ExampleFuture::YieldNow(Rc<i32>)`ステージで停止して、`Poll::Pending`を返します。
> 再度ポーリングされたとき、それは残りのコードの`println!`を実行して、`Poll::Ready(())`を返します。

When you look at its state machine representation, `ExampleFuture`,
it is now clear why `example` is not `Send`: it holds an `Rc`, therefore
it cannot be `Send`.

> その状態マシンの表現である`ExampleFuture`を確認したとき、`example`が`Send`出ない理由を明確にしています。
> それは`Rc`を保持しているため、それは`Send`になりえません。

## Yield points（yieldポイント）

As you've just seen with `example`, every `.await` point creates a new intermediate
state in the lifecycle of a future.\
That's why `.await` points are also known as **yield points**: your future _yields control_
back to the runtime that was polling it, allowing the runtime to pause it and (if necessary)
schedule another task for execution, thus making progress on multiple fronts concurrently.

> `example`で確認した通り、すべての`.await`ポイントはフューチャーのライフサイクル内に、新しい中間状態を作成します。
> それが`.await`ポイントが**yieldポイント**として知られる理由です。
> フューチャーは、それ（フューチャー）をポーリングしていたランタイムに**制御を移譲して**、ランタイムがそれ（フューチャー）を停止して、さらに必要があれば他のタスクの実行をスケジュールできるようにします。
> これにより、複数を同時並行で進めるようにします。

We'll come back to the importance of yielding in a later section.

> 後の方のセクションで、移譲することの重要性に戻ります。
