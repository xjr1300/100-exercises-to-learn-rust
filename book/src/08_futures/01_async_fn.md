# Asynchronous functions（非同期関数）

All the functions and methods you've written so far were eager.\
Nothing happened until you invoked them. But once you did, they ran to
completion: they did **all** their work, and then returned their output.

> これまで記述してきたすべての関数とメソッドは**熱心**でした。
> それらを呼び出すまで何も起こりません。しかし、一旦、呼び出すと、それらは完了するまで実行しました。
> それらは、それらの仕事を**すべて**行い、そしてそれらの出力を返しました。

Sometimes that's undesirable.\
For example, if you're writing an HTTP server, there might be a lot of
**waiting**: waiting for the request body to arrive, waiting for the
database to respond, waiting for a downstream service to reply, etc.

> 時々、それは望ましくありません。
> 例えば、HTTPサーバーを記述している場合、多くの**待機**があるかもしれません。
> リクエスト本体の到着を待つ、データベースの反応を待つ、下流サービスの応答を待つ、などです。

What if you could do something else while you're waiting?\
What if you could choose to give up midway through a computation?\
What if you could choose to prioritise another task over the current one?

> 待っている間に他になにかすることができるとしたらどうでしょうか？
> 計算の途中で諦めることを選択できるとしたらどうでしょうか？
> 現在のタスクよりも他のタスクの優先することを選択できるとしたらどうでしょうか？

That's where **asynchronous functions** come in.

> そこで**非同期関数**が登場します。

## `async fn`

You use the `async` keyword to define an asynchronous function:

> 非同期関数を定義するために`async`キーワードを使用します。

```rust
use tokio::net::TcpListener;

// This function is asynchronous
// この関数は非同期です。
async fn bind_random() -> TcpListener {
    // [...]
}
```

What happens if you call `bind_random` as you would a regular function?

> 通常の関数のように`bind_random`を呼び出した場合、何が起こるでしょうか？

```rust
fn run() {
    // Invoke `bind_random`
    // `bind_random`を呼び出します。
    let listener = bind_random();
    // Now what?
    // ここで何が起こりますか？
}
```

Nothing happens!\
Rust doesn't start executing `bind_random` when you call it,
not even as a background task (as you might expect based on your experience
with other languages).
Asynchronous functions in Rust are **lazy**: they don't do any work until you
explicitly ask them to.
Using Rust's terminology, we say that `bind_random` returns a **future**, a type
that represents a computation that may complete later. They're called futures
because they implement the `Future` trait, an interface that we'll examine in
detail later on in this chapter.

> 何も起こりません！
> Rustは、`bind_random`を呼び出したときに、それの実行を開始せず、バックグラウンドタスクとしても開始しません（他の言語の経験に基づいて予想したように）。
> Rustにおける非同期関数は**怠惰**です。
> それらは、明示的にそれらに問い合わせするまで、何の作業も行いません。
> Rustの専門用語を使用して、`bind_random`は**フューチャー**を返すといいます。
> フューチャーは、後で完了するかもしれない計算を表現する方です。
> それらは、`Future`トレイトを実装しているため、フューチャーと呼ばれます。
> `Future`トレイトのインターフェイスは、この章の後半で詳しく調べる予定です。

## `.await`

The most common way to ask an asynchronous function to do some work is to use
the `.await` keyword:

> 何らかの仕事をするために非同期関数に問い合わせする最も一般的な方法は、`.await`キーワードを使用することです。

```rust
use tokio::net::TcpListener;

async fn bind_random() -> TcpListener {
    // [...]
}

async fn run() {
    // Invoke `bind_random` and wait for it to complete
    // `bind_random`を呼び出し、それが完了するまで待機します。
    let listener = bind_random().await;
    // Now `listener` is ready
    // ここで、`lister`は準備できています。
}
```

`.await` doesn't return control to the caller until the asynchronous function
has run to completion—e.g. until the `TcpListener` has been created in the example above.

> `.await`は、非同期関数が完了まで実行されるまで、呼び出し側に制御を返しません。
> つまり、上記例において、`TcpListener`が作成されるまで制御を返しません。

## Runtimes

If you're puzzled, you're right to be!\
We've just said that the perk of asynchronous functions
is that they don't do **all** their work at once. We then introduced `.await`, which
doesn't return until the asynchronous function has run to completion. Haven't we
just re-introduced the problem we were trying to solve? What's the point?

> もし戸惑っているなら、それは正しいことです！
> 非同期関数の活性は、それらが一度にそれらの仕事を**すべて**行わないことと言いました。
> `.await`を導入して、それは非同期関数が完了まで実行されるまで戻りません。
> 解決することを試みる問題を再度導入しただけでしょうか？
> 何がポイントなのでしょうか？

Not quite! A lot happens behind the scenes when you call `.await`!\
You're yielding control to an **async runtime**, also known as an **async executor**.
Executors are where the magic happens: they are in charge of managing all your
ongoing asynchronous **tasks**. In particular, they balance two different goals:

- **Progress**: they make sure that tasks make progress whenever they can.
- **Efficiency**: if a task is waiting for something, they try to make sure that
  another task can run in the meantime, fully utilising the available resources.

> 全くそうではありません！`.await`を呼び出したとき、多くのことが背後で発生します！
> **非同期エグゼキューター**として知られる、**非同期ランタイム**に制御を譲っています。
> エグゼキューターは、そこで魔法を起こします。
> エグゼキューターは、実行中の非同期タスクをすべて管理する責任があります。
> 特に、2つの異なる目的のバランスを取ります。
>
> - **進捗**: エグゼキューターは、タスクが可能なときはいつでも、タスクを進めます。
> - **効率性**: タスクが何かを待っている場合、エグゼキューターは、その間に他のタスクを実行できるようにすることを試み、利用可能な資源を十分に活用します。

### No default runtime（デフォルトランタイムはない）

Rust is fairly unique in its approach to asynchronous programing: there is
no default runtime. The standard library doesn't ship with one. You need to
bring your own!

> Rustは、非同期プログラミングに対する方法がかなり独特です。
> デフォルトのランタイムはありません。
> 標準ライブラリは、それを一緒に提供していません。
> 独自に持ってくる必要があります！

In most cases, you'll choose one of the options available in the ecosystem.
Some runtimes are designed to be broadly applicable, a solid option for most applications.
`tokio` and `async-std` belong to this category. Other runtimes are optimised for
specific use cases—e.g. `embassy` for embedded systems.

> ほとんどの場合、エコシステムに利用できる選択肢の1つを選択することになります。
> いくつかのランタイムは、広く適用できるように設計されており、ほとんどのアプリケーションにとって堅実な選択となります。
> `tokio`と`async-std`は、このカテゴリに属しています。
> 他のランタイムは、特定のユースケースのために最適化されています。
> 例えば、`embassy`は組み込みシステム用です。

Throughout this course we'll rely on `tokio`, the most popular runtime for general-purpose
asynchronous programming in Rust.

> このコースを通じて、`tokio`に依存する予定で、Rustにおける一般的な目的の非同期プログラミングで、最も人気のあるランタイムです。

### `#[tokio::main]`

The entrypoint of your executable, the `main` function, must be a synchronous function.
That's where you're supposed to set up and launch your chosen async runtime.

> 実行形式のエントリポイントは、`main`関数で、それは非同期関数でなくてはなりません。
> それは、選択した非同期ランタイムを準備して起動することを支援されるところです。

Most runtimes provides a macro to make this easier. For `tokio`, it's `tokio::main`:

> ほとんどのランタイムは、これを簡単にするマクロを提供しています。`tokio`の場合、それは`tokio::main`です。

```rust
#[tokio::main]
async fn main() {
    // Your async code goes here
    // 非同期コードがここに入ります。
}
```

which expands to:

> それは次のように展開されます。

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        // Your async function goes here
        // [...]
    );
}
```

### `#[tokio::test]`

The same goes for tests: they must be synchronous functions.\
Each test function is run in its own thread, and you're responsible for
setting up and launching an async runtime if you need to run async code
in your tests.\
`tokio` provides a `#[tokio::test]` macro to make this easier:

> テストについても同様です。それらは同期関数でなくてはなりません。
> （記述ミス？）「それらは**非**同期関数でなくてはなりません」の誤り？
> それぞれのテスト関数は、それ自身のスレッドで実行され、テスト内で非同期コードを実行する日通用がある場合、開発者が非同期ランタイムを準備して起動する責任があります。
> `tokio`はこれを簡単にするために`#[tokio::test]`マクロを提供しています。

```rust
#[tokio::test]
async fn my_test() {
    // Your async test code goes here
}
```
