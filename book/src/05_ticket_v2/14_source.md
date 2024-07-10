# `Error::source`

There's one more thing we need to talk about to complete our coverage of the `Error` trait: the `source` method.

> `Error`トレイトの範囲を完了するために、話す必要があることがもう1つあります。それは`source`メソッドです。

```rust
// Full definition this time!
// 今回は完全な定義です。
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause**, if any.\
Errors are often chained, meaning that one error is the cause of another: you have a high-level error (e.g.
cannot connect to the database) that is caused by a lower-level error (e.g. can't resolve the database hostname).
The `source` method allows you to "walk" the full chain of errors, often used when capturing error context in logs.

> `source`メソッドは、もしある場合、**エラーの原因**にアクセスする方法です。
> 時々、エラーは連鎖され、それは1つのエラーは他の原因になることを意味します。
> 例えばデータベースのホスト名を解決できないなど、低水準のエラーによって引き起こされた、例えばデータベースに接続できないなどの高水準なエラーがあります。
> `source`メソッドは、エラーの完全なつながりを「横断」させて、時々ログでエラーのコンテキストをキャプチャーするときに使用されます。

## Implementing `source`（sourceの実装）

The `Error` trait provides a default implementation that always returns `None` (i.e. no underlying cause). That's why
you didn't have to care about `source` in the previous exercises.\
You can override this default implementation to provide a cause for your error type.

> `Error`トレイトは、例えば根本的な原因のない`None`を常に返すデフォルト実装を提供しています。
> それが、前の演習で`source`について気にする必要がなかった理由です。
> 独自のエラー型で原因を提供するためにデフォルト実装を上書きできます。

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

In this example, `DatabaseError` wraps an `std::io::Error` as its source.
We then override the `source` method to return this source when called.

> この例において、`DatabaseError`は、そのソースとして`std::io::Error`をラップしています。
> そして、呼び出されたときに、このソースを返すために`source`メソッドを上書きします。

## `&(dyn Error + 'static)`

What's this `&(dyn Error + 'static)` type?\
Let's unpack it:

- `dyn Error` is a **trait object**. It's a way to refer to any type that implements the `Error` trait.
- `'static` is a special **lifetime specifier**.
  `'static` implies that the reference is valid for "as long as we need it", i.e. the entire program execution.

> この`&(dyn Error + 'static)`型はなんでしょうか？
> それを分解してみましょう。
>
> - `dyn Error`は**トレイトオブジェクト**です。それは、`Error`トレイトを実装する任意の型を参照する方法です。
> - `'static`は特別な**ライフタイム注釈**です。
>   `'static`は、例えばプログラムの実行全体でなど、参照が「それが必要とされる限り」有効であることを暗黙的に示しています。

Combined: `&(dyn Error + 'static)` is a reference to a trait object that implements the `Error` trait
and is valid for the entire program execution.

> 組み合わせると: `&(dyn Error + 'static)`は、`Error`トレイトを実装したトレイトオブジェクトへの参照で、プログラムの実行全体で有効です。

Don't worry too much about either of these concepts for now. We'll cover them in more detail in future chapters.

> 今のところ、これらの概念についてあまり心配しないでください。将来のチャプターでより詳細にそれらを説明します。

## Implementing `source` using `thiserror`（thiserrorを使用してsourceを実装する）

`thiserror` provides three ways to automatically implement `source` for your error types:

> `thiserror`は、独自のエラー型に対して`source`を自動的に実装する3つの方法を提供します。

- A field named `source` will automatically be used as the source of the error.

  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```

- A field annotated with the `#[source]` attribute will automatically be used as the source of the error.

  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```

- A field annotated with the `#[from]` attribute will automatically be used as the source of the error **and**
  `thiserror` will automatically generate a `From` implementation to convert the annotated type into your error type.

  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

> - フィールド名`source`は、エラーのソースとして自動的に使用されます。
> - `#[source]`属性で注釈されたフィールドは、エラーのソースとして自動的に使用されます。
> - `#[from]`属性で注釈されたフィールドは、エラーのソースとして自動的に使用され、**さらに** `thiserror`は、注釈された型を独自のエラー型に変換する`From`実装を自動的に生成します。

## The `?` operator（?演算子）

The `?` operator is a shorthand for propagating errors.\
When used in a function that returns a `Result`, it will return early with an error if the `Result` is `Err`.

For example:

> `?`演算子はエラーを伝播するための少ｙ楽系です。
> `Result`を返す関数内で使用したとき、`Result`が`Err`である場合、エラーで早期リターンします。
>
> 例えば・・・

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

is equivalent to:

> 上記は次と同等です。

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

You can use the `?` operator to shorten your error handling code significantly.\
In particular, the `?` operator will automatically convert the error type of the fallible operation into the error type
of the function, if a conversion is possible (i.e. if there is a suitable `From` implementation)

> エラー処理コードを大幅に短くするために`?`演算子を使用できます。
> 特に、`?`演算子は、例えば適切な`From`実装がある場合など、変換が可能な場合、失敗する可能性がある操作のエラー型を関数のエラー型に自動的に変換します。
