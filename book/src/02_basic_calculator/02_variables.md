# Variables（変数）

In Rust, you can use the `let` keyword to declare **variables**.\
For example:

> Rustにおいて、**変数**を宣言するために`let`キーワードを使用できます。
> 例えば・・・

```rust
let x = 42;
```

Above we defined a variable `x` and assigned it the value `42`.

> 上記で変数`x`を定義して、その値に`42`を割り当てました。

## Type（型）

Every variable in Rust must have a type. It can either be inferred by the compiler or explicitly specified by the
developer.

> Rustにおいて、すべての変数は型を持たなくてはなりません。
> 型はコンパイラーによって推論されるか、開発者によって明示的に指定されるかのどちらかです。

### Explicit type annotation（明示的な型注釈）

You can specify the variable type by adding a colon `:` followed by the type after the variable name. For example:

> 変数名の後に、コロン`:`と型を追加することで、変数の型を指定できます。
> 例えば・・・

```rust
// let <variable_name>: <type> = <expression>;
let x: u32 = 42;
```

In the example above, we explicitly constrained the type of `x` to be `u32`.

> 上記の例で、`u32`で`x`の型を明示的に制約しました。

### Type inference（型推論）

If we don't specify the type of a variable, the compiler will try to infer it based on the context in which the variable
is used.

> 変数の型を指定しない場合、コンパイラーは変数が使用される文脈に基づいて推論を試みます。

```rust
let x = 42;
let y: u32 = x;
```

In the example above, we didn't specify the type of `x`.\
`x` is later assigned to `y`, which is explicitly typed as `u32`. Since Rust doesn't perform automatic type coercion,
the compiler infers the type of `x` to be `u32`—the same as `y` and the only type that will allow the program to compile
without errors.

> 上記の例で、`x`の型を指定していません。
> `x`は、後で`y`に割り当てれれ、それは明示的に`u32`型として型付けられています。
> Rustは自動的に型強制をしないため、コンパイラーは`x`の型に`u32`を推論します。
> `u32`は、`y`とエラーなしでプログラムをコンパイルできる唯一の型です。

### Inference limitations（推論の制限）

The compiler sometimes needs a little help to infer the correct variable type based on its usage.\
In those cases you'll get a compilation error and the compiler will ask you to provide an explicit type hint to
disambiguate the situation.

> 時々、コンパイラーは、変数の使用に基づいて正確に変数の型を推論するために少し助けが必要です。
> そのような場面では、コンパイルエラーが発生して、状況の曖昧さをなくすためにコンパイラーは明示的なヒントを提供するように要求します。

## Function arguments are variables（関数の引数と変数）

Not all heroes wear capes, not all variables are declared with `let`.\
Function arguments are variables too!

> すべてのヒーローがマントを付けているわけでななく、すべての変数は`let`で宣言されるわけではありません。
> 関数引数も変数です！

```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
```

In the example above, `x` is a variable of type `u32`.\
The only difference between `x` and a variable declared with `let` is that functions arguments **must** have their type
explicitly declared. The compiler won't infer it for you.\
This constraint allows the Rust compiler (and us humans!) to understand the function's signature without having to look
at its implementation. That's a big boost for compilation speed[^speed]!

> 上記例で、`x`は`u32`型の変数です。
> `x`と`let`で宣言された変数の唯一の違いは、関数の引数はそれらの型を明示的に宣言されなければならないことです。
> コンパイラーはそれを推論しません。
> この制約は、Rustコンパイラー（そして私たち人間！）が実装を確認しなくても関数のシグネチャを理解できるようにします。
> それは、コンパイル速度を大きく向上させます。

## Initialization（初期化）

You don't have to initialize a variable when you declare it.\
For example

> 変数を宣言したとき、変数を初期化する必要はありません。

```rust
let x: u32;
```

is a valid variable declaration.\
However, you must initialize the variable before using it. The compiler will throw an error if you don't:

> 上記は、妥当な変数宣言です。
> しかし、それを使用する前に変数を初期化しなくてはなりません。
> それをしない場合、コンパイラーはエラーを投げます。

```rust
let x: u32;
let y = x + 1;
```

will throw a compilation error:

> 上記はコンパイルエラーを投げます。

```text
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:3:9
  |
2 | let x: u32;
  |     - binding declared here but left uninitialized
3 | let y = x + 1;
  |         ^ `x` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 | let x: u32 = 0;
  |            +++
```

[^speed]: The Rust compiler needs all the help it can get when it comes to compilation speed.
Rustコンパイラーは、コンパイル速度に関して、可能な限りの助けを必要としています。
