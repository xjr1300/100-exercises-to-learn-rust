# Arrays（配列）

As soon as we start talking about "ticket management" we need to think about a way to store _multiple_ tickets.
In turn, this means we need to think about collections. In particular, homogeneous collections:
we want to store multiple instances of the same type.

> 「チケットの管理」について話し始めるとすぐに、_複数の_チケットを保存する方法について考える必要があります。
> 次に、これはコレクションについて考える必要があることを意味します。特に、同種のコレクション、同じ型の複数のインスタンスを保存したいと考えています。

What does Rust have to offer in this regard?

> この考えに関してRustは何を提供するでしょうか？

## Arrays（配列）

A first attempt could be to use an **array**.\
Arrays in Rust are fixed-size collections of elements of the same type.

> 最初の試みは、**配列**を使うことです。
> Rustにおける配列は、同じ型の要素の固定サイズのコレクションです。

Here's how you can define an array:

> ここに、配列を定義する方法を示します。

```rust
// Array type syntax: [ <type> ; <number of elements> ]
let numbers: [u32; 3] = [1, 2, 3];
```

This creates an array of 3 integers, initialized with the values `1`, `2`, and `3`.\
The type of the array is `[u32; 3]`, which reads as "an array of `u32`s with a length of 3".

> これは、3つの整数の配列を作成して、値`1`、`2`、`3`で初期化します。
> 配列の型は`[u32; 3]`で、それは「3の長さを持つ`u32`の配列」と読みます。

### Accessing elements（要素へのアクセス）

You can access elements of an array using square brackets:

> 角括弧を使用して配列の要素にアクセスできます。

```rust
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
Arrays are **zero-indexed**, like everything in Rust. You've seen this before with string slices and field indexing in
tuples/tuple-like variants.

> インデックスは`usize`型でなくてはなりません。
> Rustにおいてすべてのものと同様に、配列は**ゼロベース**です。
> 前の文字列スライスと、タプル／タプルのようなバリアントにおけるフィールドインデックスでこれを確認しました。

### Out-of-bounds access（範囲外アクセス）

If you try to access an element that's out of bounds, Rust will panic:

> 範囲外の要素にアクセスすることを試みた場合、Rustはパニックします。

```rust
let numbers: [u32; 3] = [1, 2, 3];
let fourth = numbers[3]; // This will panic
                         // これはパニックします。
```

This is enforced at runtime using **bounds checking**. It comes with a small performance overhead, but it's how
Rust prevents buffer overflows.\
In some scenarios the Rust compiler can optimize away bounds checks, especially if iterators are involved—we'll speak
more about this later on.

> これは**境界チェック**を使用してランタイムで強制されます。
> それは小さな性能に影響するオーバーヘッドを伴いますが、それはRustがバッファーオーバーフローを避ける方法です。
> いくつかのシナリオにおいて、特にイテレーターを含む場合、Rustコンパイラーは境界チェックを最適化できます。
> この後で詳しく説明します。

If you don't want to panic, you can use the `get` method, which returns an `Option<&T>`:

> パニックしたくない場合は、`Option<&T>`を返す`get`メソッドを使用できます。

```rust
let numbers: [u32; 3] = [1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
// 範囲外のインデックスにアクセスを試みた場合、パニックするのではなく、`None`を得ます。
assert_eq!(numbers.get(3), None);
```

### Performance（性能）

Since the size of an array is known at compile-time, the compiler can allocate the array on the stack.
If you run the following code:

> 配列のサイズはコンパイル時にわかっているため、コンパイラーはスタックに配列を割当てることができます。
> 次のコードを実行すると・・・

```rust
let numbers: [u32; 3] = [1, 2, 3];
```

You'll get the following memory layout:

> 次のメモリレイアウトが得られます。

```text
        +---+---+---+
Stack:  | 1 | 2 | 3 |
        +---+---+---+
```

In other words, the size of an array is `std::mem::size_of::<T>() * N`, where `T` is the type of the elements and `N` is
the number of elements.\
You can access and replace each element in `O(1)` time.

> 言い換えると、配列のサイズは`std::mem::size_of::<T>() * N`であり、`T`は要素の型で、｀N`は要素の数です。
> `O(1)`時間でそれぞれの要素にアクセスまたは入れ替えできます。
