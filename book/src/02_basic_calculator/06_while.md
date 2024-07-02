# Loops, part 1: `while`（ループ、その1: while）

Your implementation of `factorial` has been forced to use recursion.\
This may feel natural to you, especially if you're coming from a functional programming background.
Or it may feel strange, if you're used to more imperative languages like C or Python.

> `factorial`の実装は、再帰を使用するように強制されました。
> 特に関数プログラミングをバックグラウンドに保つ場合、これは自然に感じるかもしれません。
> また、CやPythonのようなより命令的な言語をより使用してきた場合、奇妙に感じたかもしれません。

Let's see how you can implement the same functionality using a **loop** instead.

> 代わりに**ループ**を使用して同じ機能を実装する方法を見ましょう。

## The `while` loop（whileループ）

A `while` loop is a way to execute a block of code as long as a **condition** is true.\
Here's the general syntax:

> `while`ループは、**条件**がtrueである限りコードブロックを実行する方法です。
> ここに一般的な構文を示します。

```rust
while <condition> {
    // code to execute
}
```

For example, we might want to sum the numbers from 1 to 5:

> 例えば、1から5までの数を合計したいとします。

```rust
let sum = 0;
let i = 1;
// "while i is less than or equal to 5"
// iが5より小さいか等しい間
while i <= 5 {
    // `+=` is a shorthand for `sum = sum + i`
    // `+=`は`sum = sum + i`の省略形です
    sum += i;
    i += 1;
}
```

This will keep adding 1 to `i` and `i` to `sum` until `i` is no longer less than or equal to 5.

> これは、`i`が5以下でなくなるまで、1を`i`に、`i`を`sum`に追加し続けます。

## The `mut` keyword（mutキーワード）

The example above won't compile as is. You'll get an error like:

> 上記例はコンパイルされません。次のようなエラーを得ます。

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

This is because variables in Rust are **immutable** by default.\
You can't change their value once it has been assigned.

> これは、デフォルトでRustの変数が**不変（イミュータブル）**であるためです。
> 一度、変数が値を割り当てられると、それらの値を変更できません。

If you want to allow modifications, you have to declare the variable as **mutable** using the `mut` keyword:

> 変更を許可したい場合、`mut`キーワードを使用して、**可変（ミュータブル）**として変数を宣言しなくてはなりません。

```rust
// `sum` and `i` are mutable now!
// 現在、`sum`と`i`は可変です！
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

This will compile and run without errors.

> これはコンパイルされ、エラー無しで実行されます。

## Further reading（参考資料）

- [`while` loop documentation](https://doc.rust-lang.org/std/keyword.while.html)
