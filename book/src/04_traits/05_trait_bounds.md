# Trait bounds（トレイト制約）

We've seen two use cases for traits so far:

- Unlocking "built-in" behaviour (e.g. operator overloading)
- Adding new behaviour to existing types (i.e. extension traits)

> これまで、トレイトの2つのユースケースを確認しました。
>
> - 例えば、演算子のオーバーロードなど、「組み込み機能」の振る舞いを解除する。
> - 拡張トレイトなど、既存の型に新しい振る舞いを追加する。

There's a third use case: **generic programming**.

> 3つ目のユースケースがあります。それは**ジェネリックプログラミング**です。

## The problem（課題）

All our functions and methods, so far, have been working with **concrete types**.\
Code that operates on concrete types is usually straightforward to write and understand. But it's also
limited in its reusability.\
Let's imagine, for example, that we want to write a function that returns `true` if an integer is even.
Working with concrete types, we'd have to write a separate function for each integer type we want to
support:

> これまで、すべての関数とメソッドは、**具象型**と機能してきました。
> 具象型に対して操作するコードは、通常、記述することや理解することが簡単です。
> しかし、それはその再利用性を制限されます。
> 例えば、整数が偶数である場合に`true`を返す関数を記述したいとしましょう。
> 具象型と機能する場合、サポートしたい整数型それぞれに対して、別々の関数を記述しなければなりません。

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// Etc.
```

Alternatively, we could write a single extension trait and then different implementations for each integer type:

代わりに、1つの拡張トレイトと整数型それぞれに異なる実装を記述することができます。

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// Etc.
```

The duplication remains.

> 重複は残ります。

## Generic programming（ジェネリックプログラミング）

We can do better using **generics**.\
Generics allow us to write code that works with a **type parameter** instead of a concrete type:

> **ジェネリック**を使用することで、より良くできます。
> ジェネリックは、具象型の代わりに**型パラメーター**と機能するコードを記述できるようにします。

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` is a **generic function**.\
It isn't tied to a specific input type. Instead, it works with any type `T` that:

- Implements the `IsEven` trait.
- Implements the `Debug` trait.

> `print_if_even`は、**ジェネリック関数**です。
> それは、特定の入力型に結びついていません。代わりに、それは次の任意の`T`型と機能します。
>
> - `IsEven`トレイトを実装している。
> - `Debug`トレイトを実装している。

This contract is expressed with a **trait bound**: `T: IsEven + Debug`.\
The `+` symbol is used to require that `T` implements multiple traits. `T: IsEven + Debug` is equivalent to
"where `T` implements `IsEven` **and** `Debug`".

> この契約は、`T: IsEven + Debug`という、**トレイト制約**で表現されています。
> `+`記号は、`T`が複数のトレイトを実装していることを要求するために使用されます。
> `T: IsEven + Debug`は、「`T`が`IsEven` **と** `Debug`を実装している」と同等です。

## Trait bounds（トレイト制約）

What purpose do trait bounds serve in `print_if_even`?\
To find out, let's try to remove them:

> `print_if_even`において、トレイト制約はどのような目的があるのでしょうか？
> それを見出すために、それらを削除してみましょう。

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

This code won't compile:

> このコードはコンパイルされません。

```text
error[E0599]: no method named `is_even` found for type parameter `T` in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

Without trait bounds, the compiler doesn't know what `T` **can do**.\
It doesn't know that `T` has an `is_even` method, and it doesn't know how to format `T` for printing.
From the compiler point of view, a bare `T` has no behaviour at all.\
Trait bounds restrict the set of types that can be used by ensuring that the behaviour required by the function
body is present.

> トレイト制約なしで、コンパイラーは`T`が**できること**を知りません。
> コンパイラーは、`T`が`is_even`メソッドを持っていることを知らず、出力するために`T`を書式化する方法を知りません。
> コンパイラーの観点から、素の`T`はまったく振る舞いを持っていません。
> トレイト制約は、関数本体に存在する要求された振る舞いを確実にすることで、使用できる型の集合を制約します。

## Syntax: inlining trait bounds（構文：インライントレイト制約）

All the examples above used a **`where` clause** to specify trait bounds:

> 上記例すべては、トレイト制約を指定するために**where節**を使用しました。

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  This is a `where` clause
{
    // [...]
}
```

If the trait bounds are simple, you can **inline** them directly next to the type parameter:

> トレイト制約が単純であれば、型パラメーターの次にそれらを直接**インライン**で記述できます。

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           This is an inline trait bound
    // [...]
}
```

## Syntax: meaningful names（構文：意味のある名前）

In the examples above, we used `T` as the type parameter name. This is a common convention when a function has
only one type parameter.\
Nothing stops you from using a more meaningful name, though:

> 上記例において、型パラメーター名として`T`を使用しました。
> これは、関数がたった1つの型パラメーターを持つときの慣例です。
> しかし、より意味のある名前を使用することは妨げられません。

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

It is actually **desirable** to use meaningful names when there are multiple type parameters at play or when the name
`T` doesn't convey enough information about the type's role in the function.
Maximize clarity and readability when naming type parameters, just as you would with variables or function parameters.
Follow Rust's conventions, though: use [upper camel case for type parameter names](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).

> 複数の型パラメーターが関与するときや、関数内の型の役目について、名前`T`が十分な情報を伝えていないとき、実際、意味のある名前を使用することは**望ましい**です。
> 型パラメーターに名前を付けるときの最大限の明確化と読みやすさは、ちょうど変数や関数の引数に名前を付けるときと同じです。
> ただし、型パラメーターに大文字のキャメルケースを使用する、Rustの慣習に従ってください。

## The function signature is king（関数のシグネチャーは王様）

You may wonder why we need trait bounds at all. Can't the compiler infer the required traits from the function's body?\
It could, but it won't.\
The rationale is the same as for [explicit type annotations on function parameters](../02_basic_calculator/02_variables.md#function-arguments-are-variables):
each function signature is a contract between the caller and the callee, and the terms must be explicitly stated.
This allows for better error messages, better documentation, less unintentional breakages across versions,
and faster compilation times.

> トレイト制約が必要な理由に戸惑っているかもしれません。コンパイラーは、関数の本体から要求されるトレイトを推測することができないのでしょうか？
> できるかもしれませんが、しません。
> 論理的根拠は、明示的な関数引数の型注釈と同じです。
> それぞれの関数のシグネチャーは、呼び出される側と呼び出す側の間の契約であり、その表現は明示的に述べられなければなりません。
> これは、良いエラーメッセージ、良いドキュメント、バージョン間の意図しない破壊の減少、そしてコンパイル時間の短縮を可能にします。
