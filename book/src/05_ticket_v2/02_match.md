# `match`

You may be wondering—what can you actually **do** with an enum?\
The most common operation is to **match** on it.

> 戸惑っているかもしれません。列挙型で実際に**何ができる**のでしょうか？

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // The `|` operator lets you match multiple patterns.
            // It reads as "either `Status::ToDo` or `Status::InProgress`".
            // `|`演算子は複数のパターンにマッチします。
            // それは、「`Status::ToDo`または`Status::InProgress`のどちらか」と読みます。
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

A `match` statement that lets you compare a Rust value against a series of **patterns**.\
You can think of it as a type-level `if`. If `status` is a `Done` variant, execute the first block;
if it's a `InProgress` or `ToDo` variant, execute the second block.

> `match`文は、一連の**パターン**に対してRustの値を比較させます。
> それを型レベルの`if`と考えることができます。`status`が`Done`バリアントであれば、最初のブロックを実行します。
> それが`InProgress`または`ToDo`バリアントの場合、2番目のブロックを実行します。

## Exhaustiveness（網羅的であること）

There's one key detail here: `match` is **exhaustive**. You must handle all enum variants.\
If you forget to handle a variant, Rust will stop you **at compile-time** with an error.

> ここに1つの重要な詳細があります。`match`は**網羅的**です。すべての列挙型のバリアントを処理しなければなりません。

E.g. if we forget to handle the `ToDo` variant:

> 例えば、`ToDo`バリアントを処理することを忘れた場合・・・

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

the compiler will complain:

> コンパイラーは文句を言うでしょう。

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

This is a big deal!\
Codebases evolve over time—you might add a new status down the line, e.g. `Blocked`. The Rust compiler
will emit an error for every single `match` statement that's missing logic for the new variant.
That's why Rust developers often sing the praises of "compiler-driven refactoring"—the compiler tells you
what to do next, you just have to fix what it reports.

> これは大問題です！
> コードベースは時間とともに進化します。例えば`Blocked`など、将来に新しい状態を追加するかもしれません。
> Rustコンパイラーは、新しいバリアントに対してロジックが不足していることを、それぞれの単一の`match`文に対してエラーを出力します。
> それが、Rustの開発者が時々「コンパイラー駆動のリファクタリング」を称賛する理由です。
> コンパイラーは次に何をするか伝え、単に報告されたものを修正しなければならないだけです。

## Catch-all（すべてをキャッチ）

If you don't care about one or more variants, you can use the `_` pattern as a catch-all:

> 1つ以上のバリアントに気にしない場合、すべてをキャッチするために`_`パターンを使用できます。

```rust
match status {
    Status::Done => true,
    _ => false
}
```

The `_` pattern matches anything that wasn't matched by the previous patterns.

> `_`パターンは、前のパターンによってマッチしなかったものにマッチします。
