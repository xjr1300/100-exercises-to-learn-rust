# Syntax（構文）

<div class="warning">

Don't jump ahead!\
Complete the exercise for the previous section before you start this one.\
It's located in `exercises/01_intro/00_welcome`, in the [course GitHub's repository](https://github.com/mainmatter/100-exercises-to-learn-rust).\
Use [`wr`](00_welcome.md#wr-the-workshop-runner) to start the course and verify your solutions.

> 進みすぎないでください！
> これを始める前に、前のセクションの演習を完成してください。
> それは、GitHubリポジトリのコース内の`exercises/01_intro/00_welcome`にあります。
> コースを開始して解答を確認するために`wr`を使用してください。

</div>

The previous task doesn't even qualify as an exercise, but it already exposed you to quite a bit of Rust **syntax**.
We won't cover every single detail of Rust's syntax used in the previous exercise.
Instead, we'll cover _just enough_ to keep going without getting stuck in the details.\
One step at a time!

> 前のタスクは演習としての資格すらありませんが、すでにそれはRust**構文**のかなりの部分を公開しています。
> 前の演習で使用されたRustの構文のすべての詳細をカバーしません。
> 代わりに、詳細に行き詰まることなしに進み続けるために_ちょうど十分な_説明をする予定です。
> 1度に1つずつ！

## Comments（コメント）

You can use `//` for single-line comments:

> 1行コメントには`//`を使用できます。

```rust
// This is a single-line comment
// Followed by another single-line comment
```

## Functions（関数）

Functions in Rust are defined using the `fn` keyword, followed by the function's name, its input parameters, and its
return type.
The function's body is enclosed in curly braces `{}`.

> Rustにおける関数は、`fn`キーワードを使用して定義され、関数の名前、その入力パラメーター、そして戻り値の型が続きます。
> 関数の本体は、波括弧`{}`で囲まれています。

In previous exercise, you saw the `greeting` function:

> 前の演習において、`greeting`関数を確認しました。

```rust
// `fn` <function_name> ( <input parameters> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}
```

`greeting` has no input parameters and returns a reference to a string slice (`&'static str`).

> `greeting`は入力パラメーターを持たず、文字列スライスへの参照（`&'static str`）を返します。

### Return type（戻り値の型）

The return type can be omitted from the signature if the function doesn't return anything (i.e. if it returns `()`,
Rust's unit type).
That's what happened with the `test_welcome` function:

> 戻り値の型は、例えば、Rustのユニット型である`()`を返す場合など、関数が何も返さないとき、そのシグネチャーから省略できます。
> それが、`test_welcome`関数で発生したことです。

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

The above is equivalent to:

> 上記は、次と同等です。

```rust
// Spelling out the unit return type explicitly
//                   👇
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

### Returning values（戻り値）

The last expression in a function is implicitly returned:

> 関数の最後の式は暗黙的に返されます。

```rust
fn greeting() -> &'static str {
    // This is the last expression in the function
    // Therefore its value is returned by `greeting`
    "I'm ready to learn Rust!"
}
```

You can also use the `return` keyword to return a value early:

> また、早期に値を返すために`return`キーワードを使用できます。

```rust
fn greeting() -> &'static str {
    // Notice the semicolon at the end of the line!
    return "I'm ready to learn Rust!";
}
```

It is considered idiomatic to omit the `return` keyword when possible.

> 可能なとき`return`キーワードを省略することは、慣用的である考えられています。

### Input parameters（入力パラメーター）

Input parameters are declared inside the parentheses `()` that follow the function's name.\
Each parameter is declared with its name, followed by a colon `:`, followed by its type.

> 入力パラメーターは、関数名の後に続く括弧`()`内で宣言されます。
> それぞれのパラメーターは、その名前、コロン`:`、その型を続けて宣言されます。

For example, the `greet` function below takes a `name` parameter of type `&str` (a "string slice"):

> 例えば、下の`greet`関数は、`&str`型（「文字列スライス」）の`name`パラメーターを受け取ります。

```rust
// An input parameter
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

If there are multiple input parameters, they must be separated with commas.

> 複数の入力パラメーターがある場合、それらはカンマで区切られなければなりません。

### Type annotations（型注釈）

Since we've been mentioned "types" a few times, let's state it clearly: Rust is a **statically typed language**.\
Every single value in Rust has a type and that type must be known to the compiler at compile-time.

> 数回「型」について言及したため、それを明確に述べましょう。
> Rustは**静的型付け言語**です。
> Rustにおけるすべての単独の値は型を持ち、その型はコンパイル時にコンパイラーによって理解されなければなりません。

Types are a form of **static analysis**.\
You can think of a type as a **tag** that the compiler attaches to every value in your program. Depending on the
tag, the compiler can enforce different rules—e.g. you can't add a string to a number, but you can add two numbers
together.
If leveraged correctly, types can prevent whole classes of runtime bugs.

> 型は**静的解析**の1つの形態です。
> 型を、コンパイラーがプログラム内のすべての値に添付する**タグ**と考えることができます。
> タグに依存して、コンパイラは異なるルールを強制することができます。
> 例えば、文字列と数値を足すことはできませんが、2つの数値同士であれば足せれます。
> 正確に利用されれば、型はランタイムで発生するバグの種類の全体を避けれます。
