# Copying values, pt. 2（値をコピーする、その2）

Let's consider the same example as before, but with a slight twist: using `u32` rather than `String` as a type.

> 前と同様に同じ例を考えますが、少し変更して、型に`String`ではなく`u32`を使用します。

```rust
fn consumer(s: u32) { /* */ }

fn example() {
     let s: u32 = 5;
     consumer(s);
     let t = s + 1;
}
```

It'll compile without errors! What's going on here? What's the difference between `String` and `u32`
that makes the latter work without `.clone()`?

> それはエラーなしでコンパイルされます。何が起こっているのでしょうか？
> `.clone()`なしで後者を機能させる`String`と`u32`の違いは何でしょうか？

## `Copy`

`Copy` is another trait defined in Rust's standard library:

> `Copy`は、Rust標準ライブラリに定義された別のトレイトです。

```rust
pub trait Copy: Clone { }
```

It is a marker trait, just like `Sized`.

> それは、ちょうど`Sized`のようなマーカートレイトです。

If a type implements `Copy`, there's no need to call `.clone()` to create a new instance of the type:
Rust does it **implicitly** for you.\
`u32` is an example of a type that implements `Copy`, which is why the example above compiles without errors:
when `consumer(s)` is called, Rust creates a new `u32` instance by performing a **bitwise copy** of `s`,
and then passes that new instance to `consumer`. It all happens behind the scenes, without you having to do anything.

> 型が`Copy`を実装する場合、その型の新しいインスタンスを作成するために`.clone()`を呼び出す必要はありません。
> Rustは、**暗黙的に**それを行います。
> `u32`は`Copy`を実装する型の例で、それが、エラーなしで上記例をコンパイルできた理由です。
> `consumer(s)`が呼び出されたとき、Rustは`s`の**ビット単位のコピー**を行うことで、新しい`u32`インスタンスを作成して、新しいインスタンスを`consumer`に渡します。
> それが背後で発生した全てで、何もする必要はありません。

## What can be `Copy`?（何がCopyになれるのか？）

`Copy` is not equivalent to "automatic cloning", although it implies it.\
Types must meet a few requirements in order to be allowed to implement `Copy`.

> `Copy`は、「自動クローン」と同等ではありませんが、それを暗に意味します。
> `Copy`を実装するために従わなくてはならないいくつかの要件を満たす必要があります。

First of all, it must implement `Clone`, since `Copy` is a subtrait of `Clone`.
This makes sense: if Rust can create a new instance of a type _implicitly_, it should
also be able to create a new instance _explicitly_ by calling `.clone()`.

> まず最初に、それは`Clone`を実装しなくてはならないため、`Copy`は`Clone`のサブトレイトです。
> これには意味があります。Rustがある型の新しいインスタンスを**暗黙的に**作成できる場合、それは`.clone()`を呼び出すことで**明示的に**新しいインスタンスを作成できるべきです。

That's not all, though. A few more conditions must be met:

1. The type doesn't manage any _additional_ resources (e.g. heap memory, file handles, etc.) beyond the `std::mem::size_of`
   bytes that it occupies in memory.
2. The type is not a mutable reference (`&mut T`).

> ただし、それが全てではありません。いくつかの条件がみたされなければなりません。
>
> 1. その型は、メモリ内にそれが専有する`std::mem::size_of`で得られるバイトを超えて、例えばヒープメモリ、ファイルハンドルなど、任意の _追加_ リソースを管理できません。
> 2. その型は、可変参照（`&mut T`）ではありません。

If both conditions are met, then Rust can safely create a new instance of the type by performing a **bitwise copy**
of the original instance—this is often referred to as a `memcpy` operation, after the C standard library function
that performs the bitwise copy.

> 両方の条件を満たす場合、Rustはオリジナルのインスタンスの**ビット単位のコピー**を行うことで、その型の新しいインスタンスを安全に作成できます。
> これは、ビット単位のコピーをするC標準ライブラリの関数に由来して、`memcpy`操作と呼ばれることがよくあります。

### Case study 1: `String`（事例解説1: String）

`String` is a type that doesn't implement `Copy`.\
Why? Because it manages an additional resource: the heap-allocated memory buffer that stores the string's data.

> `String`は`Copy`を実装していない型です。
> なぜでしょうか？それは、`String`が文字列のデータを保存したヒープに割り当てられたメモリバッファーを追加リソースとして管理するからです。

Let's imagine that Rust allowed `String` to implement `Copy`.\
Then, when a new `String` instance is created by performing a bitwise copy of the original instance, both the original
and the new instance would point to the same memory buffer:

> Rustが`Copy`を実装した`String`を許可したことを想像してください。
> そして、新しい`String`インスタンスが、オリジナルのインスタンスのビット単位のコピーを実行して作成されたとき、オリジナルと新しいインスタンスの両方は、同じメモリバッファーを指し示します。

```text
              s                                 copied_s
+---------+--------+----------+      +---------+--------+----------+
| pointer | length | capacity |      | pointer | length | capacity |
|  |      |   5    |    5     |      |  |      |   5    |    5     |
+--|------+--------+----------+      +--|------+--------+----------+
   |                                    |
   |                                    |
   v                                    |
 +---+---+---+---+---+                  |
 | H | e | l | l | o |                  |
 +---+---+---+---+---+                  |
   ^                                    |
   |                                    |
   +------------------------------------+
```

This is bad!
Both `String` instances would try to free the memory buffer when they go out of scope,
leading to a double-free error.
You could also create two distinct `&mut String` references that point to the same memory buffer,
violating Rust's borrowing rules.

> これは悪いです！
> 両方の`String`インスタンスがスコープ外になったとき、両方の`String`インスタンスがメモリバッファーを開放することを試み、二重開放エラーを招きます。
> また、同じメモリバッファーを指し示す2つの別の`&mut String`参照を作成でき、Rustの借用ルールに違反します。

### Case study 2: `u32`（事例解説2: u32）

`u32` implements `Copy`. All integer types do, in fact.\
An integer is "just" the bytes that represent the number in memory. There's nothing more!
If you copy those bytes, you get another perfectly valid integer instance.
Nothing bad can happen, so Rust allows it.

> `u32`は`Copy`を実装しています。実際、すべての整数型はそうです。
> 整数は、メモリ内で数を表現する「単なる」バイトです。それ以上のことはありません。
> それらのバイトをコピーして、他の完全に有効な整数インスタンスを得られます。
> 何も悪いことは発生しないため、Rustはそれを許可します。

### Case study 3: `&mut u32`（事例解説3: &mut u32）

When we introduced ownership and mutable borrows, we stated one rule quite clearly: there
can only ever be _one_ mutable borrow of a value at any given time.\
That's why `&mut u32` doesn't implement `Copy`, even though `u32` does.

> 所有権と可変参照を導入したとき、とても明確な1つのルールを述べました。
> 任意の時点で、ただ _1つ_ の値の可変参照のみが存在できます。
> それが、`&mut u32`が`Copy`を実装しない理由で、ただし`u32`は実装しています。

If `&mut u32` implemented `Copy`, you could create multiple mutable references to
the same value and modify it in multiple places at the same time.
That'd be a violation of Rust's borrowing rules!
It follows that `&mut T` never implements `Copy`, no matter what `T` is.

> `&mut u32`が`Copy`を実装した場合、同じ値への複数の可変参照を作成でき、同時に複数の箇所でそれを修正できます。
> それは、Rustの借用ルールを違反します！
> それが、`&mut T`が`Copy`を実装しない理由で、`T`が何であれそうです。

## Implementing `Copy`（Copyの実装）

In most cases, you don't need to manually implement `Copy`.
You can just derive it, like this:

> ほとんどの場合で、手動で`Copy`を実装する必要はありません。
> 次のように、単純にそれを導出できます。

```rust
#[derive(Copy, Clone)]
struct MyStruct {
    field: u32,
}
```
