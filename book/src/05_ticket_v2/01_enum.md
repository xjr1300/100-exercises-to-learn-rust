# Enumerations（列挙型）

Based on the validation logic you wrote [in a previous chapter](../03_ticket_v1/02_validation.md),
there are only a few valid statuses for a ticket: `To-Do`, `InProgress` and `Done`.\
This is not obvious if we look at the `status` field in the `Ticket` struct or at the type of the `status`
parameter in the `new` method:

> 前のチャプターで記述した検証ロジックに基づいて、チケットの有効な状態は、`To-Do`、`InProgress`そして`Done`のみがあります。
> `Ticket`奥蔵帯の`status`フィールドや`new`メソッドの`status`の型を見ても、これは明確ではありません。

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Self {
        // [...]
    }
}
```

In both cases we're using `String` to represent the `status` field.
`String` is a very general type—it doesn't immediately convey the information that the `status` field
has a limited set of possible values. Even worse, the caller of `Ticket::new` will only find out **at runtime**
if the status they provided is valid or not.

> 両方のケースで、`status`フィールドを表現するために`String`を仕様ています。
> `String`はとても一般的な型で、`status`フィールドが可能な値の制限された集合を持っているという情報をすぐに伝えることができません。
> さらに悪いことに、`Ticket::new`の呼び出し基は、提供した状態が有効かどうかを**ランタイム**でのみ知ります。

We can do better than that with **enumerations**.

> **列挙型**を使用して、それよりもうまく行うことができます。

## `enum`

An enumeration is a type that can have a fixed set of values, called **variants**.\
In Rust, you define an enumeration using the `enum` keyword:

> 列挙型は、**バリアント**と呼ばれる固定された値の集合を持てる型です。
> Rustにおいて、`enum`キーワードを使用して列挙型を定義します。

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, just like `struct`, defines **a new Rust type**.

> ちょうど`struct`のように、`enum`は**新しいRustの型**を定義します。
