# Nullability（Null可能性）

Our implementation of the `assigned` method is fairly blunt: panicking for to-do and done tickets is far from ideal.\
We can do better using **Rust's `Option` type**.

> `assigned`メソッドの実装は、かなり切れ味が悪いです。`to-do`と`done`チケットのためにパニックすることは、理想からほど遠いです。
> **Rustの`Option`型**を使用して、うまく行えます。

## `Option`

`Option` is a Rust type that represents **nullable values**.\
It is an enum, defined in Rust's standard library:

> `Option`は**null可能な値**を表現するRustの型です。
> それは、Rust標準ライブラリに定義された列挙型です。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` encodes the idea that a value might be present (`Some(T)`) or absent (`None`).\
It also forces you to **explicitly handle both cases**. You'll get a compiler error if you are working with
a nullable value and you forget to handle the `None` case.\
This is a significant improvement over "implicit" nullability in other languages, where you can forget to check
for `null` and thus trigger a runtime error.

> `Option`は、値が存在する（`Some(T)`）、または存在しない（`None`）の考えを符号化します。
> また、それは、**明示的に両方のケースを処理**することを強制します。
> null可能な値で作業して、`None`ケースを処理することを忘れた場合、コンパイルエラーが発生します。
> これは、他の言語の「暗黙的なnull可能性」を大幅に改善して、他の言語では、`null`チェックを忘れた箇所で、ランタイムエラーを発します。

## `Option`'s definition（Optionの定義）

`Option`'s definition uses a Rust construct that you haven't seen before: **tuple-like variants**.

> `Option`の定義は、**タプルのようなバリアント**という前に見ていないRustの構成概念を使用します。

### Tuple-like variants（タプルのようなバリアント）

`Option` has two variants: `Some(T)` and `None`.\
`Some` is a **tuple-like variant**: it's a variant that holds **unnamed fields**.

> `Option`は、`Some(T)`と`None`の2つのバリアントがあります。
> `Some`は**タプルのようなバリアント**で、それは**無名のフィールド**を保持するバリアントです。

Tuple-like variants are often used when there is a single field to store, especially when we're looking at a
"wrapper" type like `Option`.

> タプルのようなバリアントは、保存するために1つのフィールドがあるとき、特に`Option`のような「ラッパー」型を見ているときに、よく使用されます。

### Tuple-like structs（タプルのような構造体）

They're not specific to enums—you can define tuple-like structs too:

> それらは列挙型に特有ではありません。タプルのような構造体も定義できます。

```rust
struct Point(i32, i32);
```

You can then access the two fields of a `Point` instance using their positional index:

> そして、それらの位置インデックスを使用して、`Point`インスタンスの2つのフィールドにアクセスできます。

```rust
let point = Point(3, 4);
let x = point.0;
let y = point.1;
```

### Tuples（タプル）

It's weird to say that something is tuple-like when we haven't seen tuples yet!\
Tuples are another example of a primitive Rust type.
They group together a fixed number of values with (potentially different) types:

> まだタプルを見ていないときに、何かがタプルのようだと言うことは奇妙です！
> タプルは、プリミティブなRustの型の別の例です。
> それらは、異なる可能性のある型の値の固定数をグループ化します。

```rust
// Two values, same type
// 2つの値は同じ型です。
let first: (i32, i32) = (3, 4);
// Three values, different types
// 3つの値は異なる型です。
let second: (i32, u32, u8) = (-42, 3, 8);
```

The syntax is simple: you list the types of the values between parentheses, separated by commas.
You can access the fields of a tuple using the dot notation and the field index:

> 構文は単純です。カッコの間に、間まで区切られた値の型をリストします。
> ドット記法とフィールドのインデックスを衣装して、タプルのフィールドにアクセスできます。

```rust
assert_eq!(second.0, -42);
assert_eq!(second.1, 3);
assert_eq!(second.2, 8);
```

Tuples are a convenient way of grouping values together when you can't be bothered to define a dedicated struct type.

> 専用の構造体型を定義することが面倒なとき、タプルは値をグループ化する便利な方法です。
