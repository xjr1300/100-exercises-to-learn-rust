# Threads（スレッド）

Before we start writing multithreaded code, let's take a step back and talk about what threads are
and why we might want to use them.

> マルチスレッドなコードの記述を開始する前に、一歩戻ってスレッドとは何か、そしてそれらを使用したい理由について話しましょう。

## What is a thread?（スレッドとは？）

A **thread** is an execution context managed by the underlying operating system.\
Each thread has its own stack, instruction pointer, and program counter.

> **スレッド**は、オペレーティングシステムの下で、管理された実行文脈です。
> それぞれのスレッドは独自のスタック、命令ポインターそしてプログラムカウンターを持ちます。

A single **process** can manage multiple threads.
These threads share the same memory space, which means they can access the same data.

> 単一**プロセス**は、複数のスレッドを管理できます。
> これらのスレッドは、同じメモリ空間を共有するため、それらは同じデータにアクセスできることを意味します。

Threads are a **logical** construct. In the end, you can only run one set of instructions
at a time on a CPU core, the **physical** execution unit.\
Since there can be many more threads than there are CPU cores, the operating system's
**scheduler** is in charge of deciding which thread to run at any given time,
partitioning CPU time among them to maximize throughput and responsiveness.

> スレッドは**論理的な**構造物です。最終的に、**物理的な**実行ユニットであるCPUコアで一度に1つの命令セットしか実行できません。
> CPUコアよりも多くのスレッドが存在できるため、オペレーティングシステムの**スケジューラー**は、与えられた時間に実行するスレッドを決定する責任があり、最大のスループットと応答性を確保するために、スレッド間でCPU時間を分配します。

## `main`

When a Rust program starts, it runs on a single thread, the **main thread**.\
This thread is created by the operating system and is responsible for running the `main`
function.

> Rustプログラムが開始したとき、それは**メインスレッド**という単一スレッドで実行されます。
> このスレッドはオペレーティングシステムによって作成され、`main`関数を実行する責任があります。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Rust's standard library provides a module, `std::thread`, that allows you to create
and manage threads.

> Rust標準ライブラリは、スレッドを作成または管理する`std::thread`モジュールを提供します。

### `spawn`

You can use `std::thread::spawn` to create new threads and execute code on them.

For example:

> 新しいスレッドを作成して、それらの上でコードを実行するために`std::thread::spawn`を使用できます。
>
> 例えば・・・

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

If you execute this program on the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa)
you'll see that the main thread and the spawned thread run concurrently.\
Each thread makes progress independently of the other.

> このプログラムをRust playgroundで実行した場合、メインスレッドと生み出されたスレッドが同時並行で実行することを確認できます。
> それぞれのスレッドは、他と独立して進行します。

> このプログラムは、`main`関数末尾にある無限ループを抜けることがないため、永遠に実行を継続する。

### Process termination（プロセスの終了）

When the main thread finishes, the overall process will exit.\
A spawned thread will continue running until it finishes or the main thread finishes.

> メインスレッドが終了したとき、プロセス全体が終了します。
> 生み出されたスレッドは、それが終了する、またはメインスレッドが終了するまで実行をつつけます。

> 下のプログラムは、`main`関数末尾にある5秒間のスリープが終わると、`main`関数が終了するため、別スレッドも同時に終了する。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

In the example above, you can expect to see the message "Hello from a thread!" printed roughly five times.\
Then the main thread will finish (when the `sleep` call returns), and the spawned thread will be terminated
since the overall process exits.

> 上記例において、大まかに5️回表示される「Hello from a thread!」を見ることを期待できます。
> そして、`sleep`呼び出しが戻ったとき、メインスレッドが終了して、プロセス全体が終了するため、生み出されたスレッドも終了します。

### `join`

You can also wait for a spawned thread to finish by calling the `join` method on the `JoinHandle` that `spawn` returns.

> `spawn`が返す`JoinHandle`に対して`join`メソッドを呼び出すことで、生み出されたスレッドが終了するまで待つこともできます。

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

In this example, the main thread will wait for the spawned thread to finish before exiting.\
This introduces a form of **synchronization** between the two threads: you're guaranteed to see the message
"Hello from a thread!" printed before the program exits, because the main thread won't exit
until the spawned thread has finished.

> この例において、メインスレッドは終了する前に生み出されたスレッドが終了するまで待ちます。
> これは、2つのスレッド間の**同期**の構造を導入します。
> 生み出されたスレッドが終了するまで、メインスレッドが終了しないため、プログラムが終了する前に「Hello from a thread!」メッセージを見ることが保証されます。
