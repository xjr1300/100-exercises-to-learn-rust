# `TryFrom` and `TryInto`（TryFromとTryInto）

In the previous chapter we looked at the [`From` and `Into` traits](../04_traits/09_from.md),
Rust's idiomatic interfaces for **infallible** type conversions.\
But what if the conversion is not guaranteed to succeed?

> 前の演習において、Rustの**失敗しない**型変換の慣用的なインターフェイスである、`From`と`Into`トレイトを確認しました。
> しかし、成功することが保証されていない変換の場合はどうなるでしょうか？

We now know enough about errors to discuss the **fallible** counterparts of `From` and `Into`:
`TryFrom` and `TryInto`.

> 現在、`From`と`Into`の**失敗する可能性がある**に対応する`TryFrom`と`TryInto`について議論できるほど、エラーを十分に理解しています。

## `TryFrom` and `TryInto`（TryFromとTryInto）

Both `TryFrom` and `TryInto` are defined in the `std::convert` module, just like `From` and `Into`.

> `TryFrom`と`TryInto`両方とも、ちょうど`From`と`Into`と同様に、`std::convert`モジュールに定義されています。

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

The main difference between `From`/`Into` and `TryFrom`/`TryInto` is that the latter return a `Result` type.\
This allows the conversion to fail, returning an error instead of panicking.

> `From`/`Into`と`TryFrom`/`TryInto`の主な違いは、後者が`Result`型を返すことです。
> これは、変換が失敗することを許可して、パニックの代わりにエラーを返します。

## `Self::Error`

Both `TryFrom` and `TryInto` have an associated `Error` type.
This allows each implementation to specify its own error type, ideally the most appropriate for the conversion
being attempted.

> `TryFrom`と`TryInto`両方は、関連型`Error`があります。
> これは、それぞれの実装に、変換の試みに対して理想的に最も適切な独自のエラー型を指定させます。

`Self::Error` is a way to refer to the `Error` associated type defined in the trait itself.

> `Self::Error`は、それ自身のトレイトに定義された`Error`関連型を参照する方法です。

## Duality（二重性）

Just like `From` and `Into`, `TryFrom` and `TryInto` are dual traits.\
If you implement `TryFrom` for a type, you get `TryInto` for free.

> `From`と`Into`と同様に、`TryFrom`と`TryInto`は双対トレイトです。
> 型に`TryFrom`を実装すると、`TryInto`が無料で提供されます。
