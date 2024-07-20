# Scoped threads（スコープで制限されたスレッド）

All the lifetime issues we discussed so far have a common source:
the spawned thread can outlive its parent.\
We can sidestep this issue by using **scoped threads**.

> これまでに議論したすべてのライフタイムの問題は、よくある原因を持っています。
> それは、生み出されたスレッドは、その親よりも長生きできることです。
> **スコープで制限されたスレッド**を使用して、この問題を回避できます。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```

Let's unpack what's happening.

> 何が起こっているか調べてみましょう。

## `scope`

The `std::thread::scope` function creates a new **scope**.\
`std::thread::scope` takes as input a closure, with a single argument: a `Scope` instance.

> `std::thread::scope`関数は新しい**スコープ**を作成します。
> `std::thread::scope`は、入力として`Scope`インスタンスを単一の引数にもつクロージャーを受け取ります。

## Scoped spawns（スコープで制限された生み出し）

`Scope` exposes a `spawn` method.\
Unlike `std::thread::spawn`, all threads spawned using a `Scope` will be
**automatically joined** when the scope ends.

> `Scope`は`spawn`メソッドを公開します。
> `std::thread::spawn`と異なり、`Scope`を使用して生み出されたすべてのスレッドは、スコープが終了したとき**自動的に結合**されます。

> `Scope`内で生み出されたスレッドは結合、つまりスレッドの終了を待つ。
> よって、`Scope`と同じスコープにある変数は、`Scope`と同じだけ生存するため、`Scope`内で生み出されたスレッドで自由に参照できる。

If we were to "translate" the previous example to `std::thread::spawn`,
it'd look like this:

> 前の例を`std::thread::spawn`に「翻訳」すると、次のようになります。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```

## Borrowing from the environment（環境から借用する）

The translated example wouldn't compile, though: the compiler would complain
that `&v` can't be used from our spawned threads since its lifetime isn't
`'static`.

> ただし、翻訳した例はコンパイルされません。
> コンパイラーは、`&v`のライフタイムが`'static`でないため、生み出したスレッドでそれを使用できないことに文句を言うでしょう。

That's not an issue with `std::thread::scope`—you can **safely borrow from the environment**.

> それは、`std::thread::scope`で問題ではありません。**環境から安全に借用**できます。

In our example, `v` is created before the spawning points.
It will only be dropped _after_ `scope` returns. At the same time,
all threads spawned inside `scope` are guaranteed to finish _before_ `scope` returns,
therefore there is no risk of having dangling references.

> 例において、`v`は生み出された地点の前に作成されます。
> それは、`scope`が戻った _後_ にのみドロップされます。
> それと同時に、`scope`内で生成されたすべてのスレッドは、`scope`が戻る _前_ に終了することが保証されているため、ダングリング参照を持つリスクはありません。

The compiler won't complain!

> コンパイラーは文句を言いません。
