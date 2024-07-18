# Traits（トレイト）

Let's look again at our `Ticket` type:

`Ticket`型を再度確認しましょう。

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

All our tests, so far, have been making assertions using `Ticket`'s fields.

> これまで、すべてのテストは、`Ticket`のフィールドを使用してアサーションしてきました。

```rust
assert_eq!(ticket.title(), "A new title");
```

What if we wanted to compare two `Ticket` instances directly?

> 直接2つの`Ticket`インスタンスを比較したい場合、どうすればいいでしょうか？

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

The compiler will stop us:

> コンパイラーは止めます。

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` is a new type. Out of the box, there is **no behavior attached to it**.\
Rust doesn't magically infer how to compare two `Ticket` instances just because they contain `String`s.

> `Ticket`は新しい型です。組み込みで、**それには振る舞いが付属していません**。
> Rustは、`Ticket`が`String`を含んでいるため、2つの`Ticket`インスタンスを比較する方法を魔法のように推測しません。

The Rust compiler is nudging us in the right direction though: it's suggesting that we might be missing an implementation
of `PartialEq`. `PartialEq` is a **trait**!

> しかし、Rustコンパイラーは、正しい方向へそっと突いています。それは`PartialEq`の実装が不足しているかもしれないことを示唆しています。
> `PartialEq`は**トレイト**です！

## What are traits?（トレイトとは？）

Traits are Rust's way of defining **interfaces**.\
A trait defines a set of methods that a type must implement to satisfy the trait's contract.

> トレイトは、**インターフェイス**を定義するRustの方法です。
> トレイトは、トレイトの契約を満たすために、型が実装しなくてはならないメソッドの集合を定義します。

### Defining a trait（トレイトの定義）

The syntax for a trait definition goes like this:

> トレイトを定義する構文は次のようになります。

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

We might, for example, define a trait named `MaybeZero` that requires its implementors to define an `is_zero` method:

> 例えば、`is_zero`メソッドを定義するために、実装者にそれを要求する`MaybeZero`と名付けられたトレイトを定義するかもしれません。

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### Implementing a trait（トレイトの実装）

To implement a trait for a type we use the `impl` keyword, just like we do for regular[^inherent] methods,
but the syntax is a bit different:

> 型にトレイトを実装するために、ちょうど通常のメソッドでしたように、`impl`キーワードを使用しますが、構文は少し異なります。

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Method body
    }
}
```

For example, to implement the `MaybeZero` trait for a custom number type, `WrappingU32`:

> 例えば、カスタムな数値型の`WrappingU32`に`MaybeZero`トレイトを実装するには・・・

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### Invoking a trait method（トレイトメソッドの呼び出し）

To invoke a trait method, we use the `.` operator, just like we do with regular methods:

トレイトメソッドを呼び出すために、ちょうど通常のメソッドでしたように、`.`演算子を使用します。

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

To invoke a trait method, two things must be true:

- The type must implement the trait.
- The trait must be in scope.

> トレイトメソッドを呼び出すために、次の2つが成立しなければなりません。
>
> - 型がそのトレイトを実装していなくてはなりません。
> - トレイトがスコープ内になければなりません。

To satisfy the latter, you may have to add a `use` statement for the trait:

> 後者を満たすために、そのトレイトのために`use`文を追加しなければならないかもしれません。

```rust
use crate::MaybeZero;
```

This is not necessary if:

- The trait is defined in the same module where the invocation occurs.
- The trait is defined in the standard library's **prelude**.
  The prelude is a set of traits and types that are automatically imported into every Rust program.
  It's as if `use std::prelude::*;` was added at the beginning of every Rust module.

> 次の場合は、これは必要ありません。
>
> - トレイトが、呼び出しが発生する同じモジュール内で定義されている。
> - トレイトが、標準ライブラリの**prelude**に定義されている。
>   `prelude`は、すべてのRustプログラム内に自動的にインポートされるトレイトや型の集合です。
>   それは、`use::std::prelude:**;`として、すべてのRustモジュールの開始に追加されたかのようです。

You can find the list of traits and types in the prelude in the
[Rust documentation](https://doc.rust-lang.org/std/prelude/index.html).

> prelude内のトレイトと型のリストは、Rustドキュメントで見つけることができます。

[^inherent]: A method defined directly on a type, without using a trait, is also known as an **inherent method**.
トレイトを使用しないで、型に直接定義されたメソッドは、**継承メソッド**とも知られています。
