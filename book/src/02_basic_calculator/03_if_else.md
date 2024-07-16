# Control flow, part 1（制御フロー、その1）

All our programs so far have been pretty straightforward.\
A sequence of instructions is executed from top to bottom, and that's it.

> これまで、すべての私達のプログラムは、とても単純でした。
> 命令のシーケンスは、上から下に実行され、それだけです。

It's time to introduce some **branching**.

> **分岐**を導入する時です。

## `if` clauses（if節）

The `if` keyword is used to execute a block of code only if a condition is true.
Here's a simple example:

> `if`キーワードは、`if`条件がtrueの場合のみ、コードブロックを実行するために使用されます。
> ここに単純な例を示します。

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```

This program will print `number is smaller than 5` because the condition `number < 5` is true.

> このプログラムは、条件`number < 5`はtrueであるため、`number is smaller than 5`を表示します。

### `else` clauses（else節）

Like most programming languages, Rust supports an optional `else` branch to execute a block of code when the condition in an
`if` expression is false.\
For example:

> ほとんどのプログラミング言語と同様に、Rustは`if`式内の条件がfalseのときにコードブロックを実行するオプションの`else`分岐をサポートしています。
> 例えば・・・

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```

## Booleans（ブーリアン）

The condition in an `if` expression must be of type `bool`, a **boolean**.\
Booleans, just like integers, are a primitive type in Rust.

> `if`式内の条件は**ブーリアン**である`bool`型にならなくてはなりません。
> ちょうど整数のようなブーリアンは、Rustにおけるプリミティブ型です。

A boolean can have one of two values: `true` or `false`.

> ブーリアンは、`true`または`false`の2つの値のうちの1つになります。

### No truthy or falsy values（真のような、また偽のような値がない）

If the condition in an `if` expression is not a boolean, you'll get a compilation error.
For example, the following code will not compile:

> `if`式内の条件がブーリアンでない場合、コンパイルエラーを得ます。
> 例えば、次のコードはコンパイルされません。

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```

You'll get the following compilation error:

> 次のコンパイルエラーが得られます。

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

This follows from Rust's philosophy around type coercion: there's no automatic conversion from non-boolean types to booleans.
Rust doesn't have the concept of **truthy** or **falsy** values, like JavaScript or Python.\
You have to be explicit about the condition you want to check.

> これは、Rustの型強制まわりの哲学に従っています。
> 非ブーリアン型からブーリアンへの自動変換はありません。
> JavaScriptまたはPythonのように、Rustは**真のような**または**偽のような**値の概念を持っていません。
> 確認したい条件について明示的でなければなりません。

### Comparison operators（比較演算子）

It's quite common to use comparison operators to build conditions for `if` expressions.\
Here are the comparison operators available in Rust when working with integers:

- `==`: equal to
- `!=`: not equal to
- `<`: less than
- `>`: greater than
- `<=`: less than or equal to
- `>=`: greater than or equal to

> `if`式の条件を構築するために比較演算子を使用することは、とても一般的です。
> ここに、整数と一緒に機能する、Rustにおいて利用できる比較演算子を示します。
>
> - `==`: 等しい
> - `!=`: 等しくない
> - `<`: より小さい
> - `>`: より大きい
> - `<=`: 以下
> - `>=`: 以上

## `if/else` is an expression（if/elseは式）

In Rust, `if` expressions are **expressions**, not statements: they return a value.\
That value can be assigned to a variable or used in other expressions. For example:

> Rustで、`if`式は**式**で、文ではありません。それらは値を返します。
> その値は変数に割当てる、または他の式の中で使用できます。
> 例えば・・・

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```

In the example above, each branch of the `if` evaluates to a string literal,
which is then assigned to the `message` variable.\
The only requirement is that both `if` branches return the same type.

> 上記例で、`if`のそれぞれの枝は、文字列リテラルに評価され、それは`message`変数に割り当てられます。
> 唯一の要求事項は、両方の`if`枝が同じ型を返すことです。
