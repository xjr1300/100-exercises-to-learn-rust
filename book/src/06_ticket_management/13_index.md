# Indexing（インデックス）

`TicketStore::get` returns an `Option<&Ticket>` for a given `TicketId`.\
We've seen before how to access elements of arrays and vectors using Rust's
indexing syntax:

> `TicketStore::get`は、与えられた`TicketId`に対して`Option<&Ticket>`を返します。
> 前に、Rustのインデックス構文を使用して、配列とベクターの要素にアクセスする方法を確認しました。

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

How can we provide the same experience for `TicketStore`?\
You guessed right: we need to implement a trait, `Index`!

> `TicketStore`に対して同じ経験を提供できるでしょうか？
> 正しい推測です。`Index`トレイ地を実装する必要があります。

## `Index`

The `Index` trait is defined in Rust's standard library:

> `Index`トレイトは、Rust標準ライブラリに定義されています。

```rust
// Slightly simplified
// 少し簡素化されています。
pub trait Index<Idx>
{
    type Output;

    // Required method
    // 要求されたメソッドです。
    fn index(&self, index: Idx) -> &Self::Output;
}
```

It has:

- One generic parameter, `Idx`, to represent the index type
- One associated type, `Output`, to represent the type we retrieved using the index

> それは次を持っています。
>
> - インデックスの型を表現する`Idx`という1つのジェネリックパラメーター
> - インデックスを使用して取得される型を表現する関連型の`Output`

Notice how the `index` method doesn't return an `Option`. The assumption is that
`index` will panic if you try to access an element that's not there, as it happens
for array and vec indexing.

> `index`メソッドが`Option`を返さないことに注意してください。
> 想定は、配列やベクターのインデックスに対して発生したように、存在しない要素にアクセスを試みた場合、`index`がパニックすることです。
