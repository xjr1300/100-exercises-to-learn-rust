# Fallibility（誤りを逃れられない性質）

Let's revisit the `Ticket::new` function from the previous exercise:

> 前の演習の`Ticket::new`関数を再度確認しましょう。

```rust
impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

As soon as one of the checks fails, the function panics.
This is not ideal, as it doesn't give the caller a chance to **handle the error**.

> 1つの検証が失敗したらすぐに、関数はパニックします。
> **エラーを処理する**機会を呼び出し側に与えないため、これは理想的ではありません。

It's time to introduce the `Result` type, Rust's primary mechanism for error handling.

> エラーを処理するためにRustの主要なメカニズムである、`Result`型を導入するときです。

## The `Result` type（Result型）

The `Result` type is an enum defined in the standard library:

> `Result`型は、標準ライブラリに定義された列挙型です。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It has two variants:

- `Ok(T)`: represents a successful operation. It holds `T`, the output of the operation.
- `Err(E)`: represents a failed operation. It holds `E`, the error that occurred.

> それは2つのバリアントがあります。
>
> - `Ok(T)`: 成功した操作を表現します。それは操作の出力である`T`を保持します。
> - `Err(E)`: 失敗した操作を表現します。それは発生したエラーである`E`を保持します。

Both `Ok` and `Err` are generic, allowing you to specify your own types for the success and error cases.

> `Ok`と`Err`の両方はジェネリックで、成功と失敗の場合の独自の型を指定できるようにします。

## No exceptions（例外はない）

Recoverable errors in Rust are **represented as values**.\
They're just an instance of a type, being passed around and manipulated like any other value.
This is a significant difference from other languages, such as Python or C#, where **exceptions** are used to signal errors.

> Rustにおいて、復旧可能なエラーは**値として表現**されます。
> 単にそれらは型のインスタンスで、他の任意の値のように、周辺に渡され、操作されます。
> これは、**例外**がエラーの合図として使用されるPythonまたはC#のような他の言語と大きく異なります。

Exceptions create a separate control flow path that can be hard to reason about.\
You don't know, just by looking at a function's signature, if it can throw an exception or not.
You don't know, just by looking at a function's signature, **which** exception types it can throw.\
You must either read the function's documentation or look at its implementation to find out.

> 例外は、理由を知ることが難しくなる分離した制御フローのパスを作成します。
> 関数が例外を投げるか投げないかは、関数のシグネチャーを見てもわかりまっせん。
> 関数がどの例外を投げるかは、関数のシグネチャーを見てもわかりません。
> 理由を見つけるために、関数のドキュメントまたはその実装のどちらかをよまなくてはなりません。

Exception handling logic has very poor locality: the code that throws the exception is far removed from the code
that catches it, and there's no direct link between the two.

> 例外処理ロジックは、とても貧弱な局所性があります。例外を投げるコードは、それを受け取るコードから遠く離れており、それは2つの間を直接結びつけられません。

## Fallibility is encoded in the type system（誤りを逃れられない性質は型システム内に符号化される）

Rust, with `Result`, forces you to **encode fallibility in the function's signature**.\
If a function can fail (and you want the caller to have a shot at handling the error), it must return a `Result`.

> `Result`を持つRustは、**関数シグネチャーに誤りを逃れられない性質を符号化**することを強制します。
> 関数が失敗する可能性があり、エラーを処理する機会を呼び出し側に与えたい場合、それは`Result`を返さなくてはなりません。

```rust
// Just by looking at the signature, you know that this function can fail.
// You can also inspect `ParseIntError` to see what kind of failures to expect.
// シグネチャーを見ただけで、この関数が失敗する可能性があることがわかります。
// また、期待する失敗の種類が何かを確認するために、`ParseIntError`を調査することもできます。
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

That's the big advantage of `Result`: it makes fallibility explicit.

> それは`Result`の大きな利点です。それは誤りを逃れられない性質を明示的にします。

Keep in mind, though, that panics exist. They aren't tracked by the type system, just like exceptions in other languages.
But they're meant for **unrecoverable errors** and should be used sparingly.

> ただし、パニックが存在することを忘れないでください。それらは型システムによって追跡されず、他の言語の例外とちょうど同じです。
> しかし、それらは**回復不可能なエラー**を意図しており、控えめに使用されるべきです。
