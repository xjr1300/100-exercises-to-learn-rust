# `'static`

If you tried to borrow a slice from the vector in the previous exercise,
you probably got a compiler error that looks something like this:

> 前の演習において、ベクターからスライスを借用することを試みた場合、おそらく次のようなコンパイルエラーが発生しました。

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = thread::spawn(move || left.iter().sum::<i32>());
   |                        ------------------------------------------------
                          argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

`argument requires that v is borrowed for 'static`, what does that mean?

> `argument requires that v is borrowed for 'static`は、何を意味しているのでしょうか？

The `'static` lifetime is a special lifetime in Rust.\
It means that the value will be valid for the entire duration of the program.

> Rustにおいて、`'static`ライフタイムは特別なライフタイムです。
> それは、その値がプログラム全体の間で有効であることを意味します。

## Detached threads（デタッチされたスレッド）

A thread launched via `thread::spawn` can **outlive** the thread that spawned it.\
For example:

> `thread::spawn`を介して起動されたスレッドは、それを生成したスレッドよりも**長生き**することができます。
> 例えば・・・

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

In this example, the first spawned thread will in turn spawn
a child thread that prints a message every second.\
The first thread will then finish and exit. When that happens,
its child thread will **continue running** for as long as the
overall process is running.\
In Rust's lingo, we say that the child thread has **outlived**
its parent.

> この例において、最初に生み出されたスレッドは、次々に毎秒メッセージを表示する子スレッドを生み出します。
> そして、最初のスレッドは完了して終了します。
> それが発生したとき、子スレッドは全体のプロセスが実行されている限り**継続して実行**します。
> Rustの専門用語において、子スレッドはその親よりも**長生きしている**といいます。

> 関数`f`を`main`関数にした場合、親スレッドが終了しても、`main`関数が終了していない微小な時間がある。
> その微小時間内で、子スレッドは実行を継続する。

## `'static` lifetime（'staticライフタイム）

Since a spawned thread can:

- outlive the thread that spawned it (its parent thread)
- run until the program exits

> 生み出されたスレッドは次ができるため・・・
>
> - それを生成した親スレッドよりも長生きする。
> - プログラムが終了するまで実行する。

it must not borrow any values that might be dropped before the program exits;
violating this constraint would expose us to a use-after-free bug.\
That's why `std::thread::spawn`'s signature requires that the closure passed to it
has the `'static` lifetime:

> それ（子スレッド）は、プログラムが終了する前にドロップされるかもしれない値を借用してはなりません。
> この制約に違反することは、「解放された後に使用する」バグをさらけ出します。
> それが、`std::thread::spawn`のシグネチャーが`'static`ライフタイムを持つそれ（値）を渡されるクロージャーを要求している理由です。

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` is not (just) about references（'staticは（単に）参照についてだけではありません）

All values in Rust have a lifetime, not just references.

> Rustにおいて、参照だけでなく、すべての値がライフタイムを持っています。

In particular, a type that owns its data (like a `Vec` or a `String`)
satisfies the `'static` constraint: if you own it, you can keep working with it
for as long as you want, even after the function that originally created it
has returned.

> 特に、`Vec`または`String`のようなデータを所有する型は、`'static`制約を満たします。
> それを所有した場合、それを作成した関数が戻った後でも、望むだけそれと一緒に作業を続けることができます。

> `Vec`が`T`の参照である`&T`を格納する場合、`&T`自体の所有権をもち、`&T`が参照する`T`は`'static`である必要がある。

You can thus interpret `'static` as a way to say:

- Give me an owned value
- Give me a reference that's valid for the entire duration of the program

> よって、`'static`は次のように解釈できます。
>
> - 所有した値を与えてくれる
> - プログラム全体の期間で有効な参照を与えてくれる

The first approach is how you solved the issue in the previous exercise:
by allocating new vectors to hold the left and right parts of the original vector,
which were then moved into the spawned threads.

> 最初の方法は、以前の演習の問題を解決した方法と同じで、オリジナルのベクターの左と右の部分を保持するために新しいベクターを割り当て、それらを生み出されたスレッド内に移動しました。

## `'static` references（'static参照）

Let's talk about the second case, references that are valid for the entire
duration of the program.

> 2つ目のケース、プログラム全体の間で有効な参照について話しましょう。

### Static data（静的なデータ）

The most common case is a reference to **static data**, such as string literals:

> 最も一般的なケースは、文字列リテラルのような**静的データ**への参照です。

```rust
let s: &'static str = "Hello world!";
```

Since string literals are known at compile-time, Rust stores them _inside_ your executable,
in a region known as **read-only data segment**.
All references pointing to that region will therefore be valid for as long as
the program runs; they satisfy the `'static` contract.

> 文字列リテラルはコンパイル時にわかるため、Rustは実行形式の _内部_ にある**読み込み専用データセグメント**として知られる領域にそれらを保存します。

## Further reading（参考資料）

- [The data segment](https://en.wikipedia.org/wiki/Data_segment)
