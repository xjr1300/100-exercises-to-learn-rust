# Error trait（Errorトレイト）

## Error reporting（エラー報告）

In the previous exercise you had to destructure the `TitleError` variant to extract the error message and
pass it to the `panic!` macro.\
This is a (rudimentary) example of **error reporting**: transforming an error type into a representation that can be
shown to a user, a service operator, or a developer.

> 前の演習において、エラーメッセージを抽出して、`panic!`マクロにそれを渡すために、`TitleError`バリアントを分解しなければなりませんでした。
> これは、**エラー報告**の初歩的な例で、ユーザー、サービスオペレーターまたは開発者に表示される表現にエラー型を変換しています。

It's not practical for each Rust developer to come up with their own error reporting strategy: it'd be a waste of time
and it wouldn't compose well across projects.
That's why Rust provides the `std::error::Error` trait.

> それは、独自のエラー報告戦略を採用するそれぞれのRust開発者にとって実用的ではありません。
> それは時間を浪費して、プロジェクト間でうまく構成されません。
> それが、Rustが`std::error::Error`トレイトを提供する理由です。

## The `Error` trait（Errorトレイト）

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type
that implements the `Error` trait.
`Error` is the cornerstone of Rust's error handling story:

> `Result`の`Err`バリアントの型に制約はありませんが、`Error`トレイトを実装する型を使用することは良い実践です。
> `Error`は、Rustのエラー処理の基本理念です。

```rust
// Slightly simplified definition of the `Error` trait
// `Error`トレイトの定義を少し簡素化しています。
pub trait Error: Debug + Display {}
```

You might recall the `:` syntax from [the `Sized` trait](../04_traits/08_sized.md)—it's used to specify **supertraits**.
For `Error`, there are two supertraits: `Debug` and `Display`. If a type wants to implement `Error`, it must also
implement `Debug` and `Display`.

> `Sized`トレイトの`:`構文を思い出したかもしれません。
> それは**スーパートレイト**を指定するために使用されます。
> `Error`には、2つのスーパートレイトがあり、それらは`Debug`と`Display`です。
> 型に`Error`を実装したい場合、その型は`Debug`と`Display`を実装しなければなりません。

## `Display` and `Debug`（DisplayとDebug）

We've already encountered the `Debug` trait in [a previous exercise](../04_traits/04_derive.md)—it's the trait used by
`assert_eq!` to display the values of the variables it's comparing when the assertion fails.

> 前の演習ですでに`Debug`トレイトに遭遇しました。
> アサーションが失敗したときに、比較している変数の値を表示するために`assert_eq!`によって使用されるトレイトです。

From a "mechanical" perspective, `Display` and `Debug` are identical—they encode how a type should be converted
into a string-like representation:

> 「機械的」な観点から、`Display`と`Debug`は同一です。
> それらは、型を文字列のような表現に変換する方法を符号化しています。

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The difference is in their _purpose_: `Display` returns a representation that's meant for "end-users",
while `Debug` provides a low-level representation that's more suitable to developers and service operators.\
That's why `Debug` can be automatically implemented using the `#[derive(Debug)]` attribute, while `Display`
**requires** a manual implementation.

> 違いはそれらの _目的_ です。
> `Display`は「エンドユーザー」を意図した表現を返しますが、`Debug`は開発者やサービスオペレーターにより適した低水準な表現を提供します。
> それが、`Debug`が`#[derive(Debug)]`属性を使用して自動的に実装できる一方で、`Display`は手動の実装を**要求する**理由です。
