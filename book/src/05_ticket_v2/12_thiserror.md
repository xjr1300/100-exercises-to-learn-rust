# `thiserror`

That was a bit of detour, wasn't it? But a necessary one!\
Let's get back on track now: custom error types and `thiserror`.

> 少し回り道をしましたか？しかしそれは必要でした！
> ここで、カスタムエラー型と`thiserror`の追跡に戻りましょう。

## Custom error types（カスタムエラー型）

We've seen how to implement the `Error` trait "manually" for a custom error type.\
Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?

> カスタムエラー型に対して「手動で」`Error`トレイトを実装する方法を確認しました。
> コードベースでほとんどのエラー型に対してこれをしなければならないことを想像してください。それは多くのボイラープレートではありませんか？

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.

> カスタムエラー型の作成を簡素化するための**手続きマクロ**を提供するRustクレートの`thiserror`を使用することで、いくつかの定型文を削除できます。

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## You can write your own macros（独自のマクロを記述できます）

All the `derive` macros we've seen so far were provided by the Rust standard library.\
`thiserror::Error` is the first example of a **third-party** `derive` macro.

> これまでに見てきたすべての`derive`マクロは、Rust標準ライブラリによって提供されています。
> `thiserror::Error`は、**サードパーティ**の`derive`マクロの最初の例です。

`derive` macros are a subset of **procedural macros**, a way to generate Rust code at compile time.
We won't get into the details of how to write a procedural macro in this course, but it's important
to know that you can write your own!\
A topic to approach in a more advanced Rust course.

> `derive`マクロは、**手続きマクロ**の部分集合で、コンパイル時にRustコードを生成する方法です。
> このコースにおいて、手続きマクロを記述する方法の詳細に入るつもりはありませんが、独自に記述できることを知ることは重要です。
> それは、より高度なRustのコースで説明するトピックです。

## Custom syntax（カスタム構文）

Each procedural macro can define its own syntax, which is usually explained in the crate's documentation.
In the case of `thiserror`, we have:

- `#[derive(thiserror::Error)]`: this is the syntax to derive the `Error` trait for a custom error type, helped by `thiserror`.
- `#[error("{0}")]`: this is the syntax to define a `Display` implementation for each variant of the custom error type.
  `{0}` is replaced by the zero-th field of the variant (`String`, in this case) when the error is displayed.

> それぞれの手続マクロはそれ独自の構文を定義でき、通常、それはクレートのドキュメントで説明されています。
> `thiserror`の場合、次のようになります。
>
> - `#[derive(thiserror::Error)]`: これは、`thiserror`によって支援される、カスタムエラー型に`Error`トレイトを導出する構文です。
> - `#[error("{0}")]`: これは、カスタムエラー型のそれぞれのバリアントに対して`Display`の実装を定義する構文です。
> - `{0}`は、エラーが表示されるときに、バリアントの0番目のフィールド（この場合は`String`）に置き換えられます。
