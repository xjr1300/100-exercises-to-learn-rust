# Modelling A Ticket, pt. 2（チケットのモデリング、その2）

The `Ticket` struct we worked on in the previous chapters is a good start,
but it still screams "I'm a beginner Rustacean!".

> 前の章で作業した`Ticket`構造体は、良いスタートですが、それでも「私は初心者のRustaceanです！」と叫んでいます。

We'll use this chapter to refine our Rust domain modelling skills.
We'll need to introduce a few more concepts along the way:

- `enum`s, one of Rust's most powerful features for data modeling
- The `Option` type, to model nullable values
- The `Result` type, to model recoverable errors
- The `Debug` and `Display` traits, for printing
- The `Error` trait, to mark error types
- The `TryFrom` and `TryInto` traits, for fallible conversions
- Rust's package system, explaining what's a library, what's a binary, how to use third-party crates

> Rustとメインモデリング技術を洗練するためにこの章を使用します。
> トチュウでいくつかの概念を紹介する必要があります。
>
> - Rustのデータモデリングの最も強力な機能の1つである`enum`
> - null可能な値をモデル化する`Option`型
> - 回復可能なエラーをモデル化する`Result`型
> - 印字するための`Debug`と`Display`トレイト
> - エラー型を作成する`Error`トレイト
> - 失敗する可能性のある変換用おの`TryFrom`と`TryInto`トレイト
> - ライブラリとは何か、バインなりとはなにか、どのようにサードパーティのクレートを使用するかを説明するRustのパッケージシステム
