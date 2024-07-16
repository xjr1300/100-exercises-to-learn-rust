# Types, part 1（型、その1）

In the ["Syntax" section](../01_intro/01_syntax.md) `compute`'s input parameters were of type `u32`.\
Let's unpack what that _means_.

> 「構文」節において、`compute`の入力パラメーターは`u32`型でした。
> それが何を意味するか解説しましょう。

## Primitive types（プリミティブ型）

`u32` is one of Rust's **primitive types**. Primitive types are the most basic building blocks of a language.
They're built into the language itself—i.e. they are not defined in terms of other types.

> `u32`はRustの**プリミティブ型**の1つです。
> プリミティブ型は言語の最も基本的な構成要素です。
> それらは、言語自体に組み込まれており、つまり他の型に基づいて定義されていません。

You can combine these primitive types to create more complex types. We'll see how soon enough.

> より複雑な型を作成するために、これらのプリミティブ型を組み合わせれます。
> すぐにその方法を確認します。

## Integers（整数）

`u32`, in particular, is an **unsigned 32-bit integer**.

> 特に`u32`は**符号なし32ビット整数**です。

An integer is a number that can be written without a fractional component. E.g. `1` is an integer, while `1.2` is not.

> 整数は、小数部分なしで記述できる数値です。
> 例えば、`1`は整数ですが、`1.2`は整数ではありません。

### Signed vs. unsigned（符号付きと符号なし）

An integer can be **signed** or **unsigned**.\
An unsigned integer can only represent non-negative numbers (i.e. `0` or greater).
A signed integer can represent both positive and negative numbers (e.g. `-1`, `12`, etc.).

> 整数は**符号付き**または**符号なし**になります。
> 符号なし整数は、例えば0またはそれより大きい非負数のみを表現できます。
> 符号付き整数は、正と負の数の両方を表現できます。

The `u` in `u32` stands for **unsigned**.\
The equivalent type for signed integer is `i32`, where the `i` stands for integer (i.e. any integer, positive or
negative).

> `u32`の`u`は**符号なし**に由来します。
> 符号付き整数と同等の型は`i32`で、`i`は整数（例えば、正または負の任意の整数）に由来します。

### Bit width（ビット幅）

The `32` in `u32` refers to the **number of bits[^bit]** used to represent the number in memory.\
The more bits, the larger the range of numbers that can be represented.

> `u32`の`32`は、メモリ内でその数を表現するために使用される**ビット数**を指します。
> ビット数が多いほど、表現できる数値の範囲が広がります。

Rust supports multiple bit widths for integers: `8`, `16`, `32`, `64`, `128`.

> Rustは、整数に対して`8`、`16`、`32`、`64`、`128`の複数のビット幅をサポートしています。

With 32 bits, `u32` can represent numbers from `0` to `2^32 - 1` (a.k.a. [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)).\
With the same number of bits, a signed integer (`i32`) can represent numbers from `-2^31` to `2^31 - 1`
(i.e. from [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN)
to [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)).\
The maximum value for `i32` is smaller than the maximum value for `u32` because one bit is used to represent
the sign of the number. Check out the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
representation for more details on how signed integers are represented in memory.

> 32ビット幅の場合、`u32`は`0`から`2^32 - 1`（`u32::MAX`として知られる）までの数値を表現できます。
> 同じビット数の場合、符号付き整数（`i32`）は`-2^31`から`2^31 - 1`（`i32::MIN`から`i32::MAX`）までの数値を表現できます。
> `i32`の最大値は`u32`の最大値よりも小さいです。なぜなら、1ビットが数値の符号を表現するために使用されるからです。
> メモリ内で符号付き整数を表現する方法の詳細は、`2の補数`表現を参照してください。

### Summary（まとめ）

Combining the two variables (signed/unsigned and bit width), we get the following integer types:

> 符号付き／符号なしとビット幅の2つの変数の組み合わせで、次の整数型が得られます。

| Bit width | Signed | Unsigned |
| --------- | ------ | -------- |
| 8-bit     | `i8`   | `u8`     |
| 16-bit    | `i16`  | `u16`    |
| 32-bit    | `i32`  | `u32`    |
| 64-bit    | `i64`  | `u64`    |
| 128-bit   | `i128` | `u128`   |

## Literals（リテラル）

A **literal** is a notation for representing a fixed value in source code.\
For example, `42` is a Rust literal for the number forty-two.

> **リテラル**は、ソースコード内で固定された値を表現するための表記法です。
> 例えば、`42`は数値42のRustのリテラルです。

### Type annotations for literals（リテラルの型注釈）

But all values in Rust have a type, so... what's the type of `42`?

> しかし、Rustにおけるすべての値は型を持つため、`42`の型は何でしょうか？

The Rust compiler will try to infer the type of a literal based on how it's used.\
If you don't provide any context, the compiler will default to `i32` for integer literals.\
If you want to use a different type, you can add the desired integer type as a suffix—e.g. `2u64` is a 2 that's
explicitly typed as a `u64`.

> Rustのコンパイラーは、それが使用される方法に基づいてリテラルの型を推論することを試みます。
> 何らかの文脈を提供しない場合、コンパイラーは整数リテラルに対してデフォルトで`i32`を与えます。
> 異なる型を使用したい場合、接尾辞として望ましい整数型を追加できます。
> 例えば、`2u64`は、`u64`として明示的に型付けられた2です。

### Underscores in literals（リテラル内のアンダースコア）

You can use underscores `_` to improve the readability of large numbers.\
For example, `1_000_000` is the same as `1000000`.

> 大きな数の可読性を改善するために、アンダースコア`_`を使用できます。
> 例えば、`1_000_000`は`1000000`と同じです。

## Arithmetic operators（算術演算子）

Rust supports the following arithmetic operators[^traits] for integers:

- `+` for addition
- `-` for subtraction
- `*` for multiplication
- `/` for division
- `%` for remainder

> Rustは整数のために次の算術演算子をサポートしています。
>
> - `+`: 加算
> - `-`: 減算
> - `*`: 乗算
> - `/`: 除算
> - `%`: 剰余

Precedence and associativity rules for these operators are the same as in mathematics.\
You can use parentheses to override the default precedence. E.g. `2 * (3 + 4)`.

> これらの演算子の優先順位と結合性のルールは、数学と同じです。
> デフォルトの優先順位を上書きするためにカッコを使用できます。
> 例えば、`2 * (3 + 4)`。

> ⚠️ **Warning**
>
> The division operator `/` performs integer division when used with integer types.
> I.e. the result is truncated towards zero. For example, `5 / 2` is `2`, not `2.5`.

> ⚠️ **警告**
> 除算演算子`/`は、整数型と使用すると整数除算を行います。
> つまり、結果はゼロに向かって切り捨てられます。
> 例えば、`5 / 2`は`2`であり、`2.5`ではありません。

## No automatic type coercion（自動型強制なし）

As we discussed in the previous exercise, Rust is a statically typed language.\
In particular, Rust is quite strict about type coercion. It won't automatically convert a value from one type to
another[^coercion],
even if the conversion is lossless. You have to do it explicitly.

> 前の演習で議論したように、Rustは静的型付け言語です。
> 特に、Rustは、型強制についてかなり厳格です。
> それは、変換が損失なしでも、値をある型から他の型に自動で変換しません。
> 明示的にそれを行わなければなりません。

For example, you can't assign a `u8` value to a variable with type `u32`, even though all `u8` values are valid `u32`
values:

> 例えば、すべての`u8`値は有効な`u32`値にも関わらず、`u32`型である変数に`u8`値を割り当てることはできません。

```rust
let b: u8 = 100;
let a: u32 = b;
```

It'll throw a compilation error:

> それはコンパイルエラーを投げます。

```text
error[E0308]: mismatched types
  |
3 |     let a: u32 = b;
  |            ---   ^ expected `u32`, found `u8`
  |            |
  |            expected due to this
  |
```

We'll see how to convert between types [later in this course](../04_traits/09_from.md).

> 型の変換方法はこのコースの後半で確認します。

## Further reading（参考文献）

- [The integer types section](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) in the official Rust book

> - 整数型節: 公式Rustブック

[^bit]: A bit is the smallest unit of data in a computer. It can only have two values: `0` or `1`.コンピューター内の最も小さなデータ単位です。それは`0`または`1`の2つの値しか持つことができません。

[^traits]: Rust doesn't let you define custom operators, but it puts you in control of how the built-in operators
behave.
We'll talk about operator overloading [later in the course](../04_traits/03_operator_overloading.md), after we've covered traits.
Rustは独自の演算子を定義することを許可しませんが、組み込み演算子の振る舞いを制御する事ができます。
コースの後半でトレイトを説明した後、演算子のオーバーロードについて話します。

[^coercion]: There are some exceptions to this rule, mostly related to references, smart pointers and ergonomics. We'll
cover those [later on](../04_traits/07_deref.md).
A mental model of "all conversions are explicit" will serve you well in the meantime.
このルールにはいくつかの例外があり、多くは参照、スマートポインターそして人間工学に関連します。それらは後で説明します。
「すべての変換は明示的である」というメンタルモデルは、それまでの間に役立ちます。
