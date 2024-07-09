# Unwrapping（包みを解く）

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
What does this mean for the caller?

> 現在、`Ticket::new`は、無効な入力に対して`Result`を返します。
> これは呼び出し側にとって何を意味するのでしょうか？

## Failures can't be (implicitly) ignored（逃れられない失敗は、暗黙的に無視できない）

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.

> 例外と異なり、Rustの`Result`は、**呼び出し側でエラーを処理する**ことを強制します。
> `Result`を返す関数を呼び出した場合、Rustはエラーケースを暗黙的に無視させません。

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// This won't compile: we're not handling the error case.
// We must either use `match` or one of the combinators provided by `Result`
// to "unwrap" the success value or handle the error.
// これはコンパイルされません。エラーケースを処理していないからです。
// 成功値の「包を解く」か、エラーを処理するために、`Result`によって提供される`match`またはコンビネーターの1つのどちらかを使用しなくてはなりません。
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?（Resultを得ました。ここで何をすればいいのでしょうか？）

When you call a function that returns a `Result`, you have two key options:

> `Result`を返す関数を呼び出したとき、主に2つの選択肢があります。

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.

  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```

- Destructure the `Result` using a `match` expression to deal with the error case explicitly.

  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```

> - 操作が失敗した場合パニックします。これは`unwrap`または`expect`メソッドのどちらかを使用して行います。
> - 明示的にエラーケースを扱うために、`match`式を使用して、`Result`を分解します。
