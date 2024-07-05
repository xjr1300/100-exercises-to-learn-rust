# Implementing traits（トレイトの実装）

When a type is defined in another crate (e.g. `u32`, from Rust's standard library), you
can't directly define new methods for it. If you try:

> 例えば、Rust標準ライブラリの`u32`など、型が他のクレートに定義されているとき、それに直接新しいメソッドを定義できません。
> それを試みると・・・

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

the compiler will complain:

> コンパイラは文句を言うでしょう。

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## Extension trait（拡張トレイト）

An **extension trait** is a trait whose primary purpose is to attach new methods
to foreign types, such as `u32`.
That's exactly the pattern you deployed in the previous exercise, by defining
the `IsEven` trait and then implementing it for `i32` and `u32`. You are then
free to call `is_even` on those types as long as `IsEven` is in scope.

> **拡張トレイト**は、`u32`のような外部の型に新しいメソッドを取り付けることが主な目的のトレイトです。
> これは、`IsEven`トレイトを定義して、そして`i32`と`u32`に実装した、まさしく前の演習で展開したパターンです。
> そして、`IsEven`がスコープ内にある限り、それらの型の`is_even`呼び出しは自由です。

```rust
// Bring the trait in scope
// トレイトをスコープ内に持ち込みます。
use my_library::IsEven;

fn main() {
    // Invoke its method on a type that implements it
    // それを実装した型のそのメソッドを呼び出します。
    if 4.is_even() {
        // [...]
    }
}
```

## One implementation（1つの実装）

There are limitations to the trait implementations you can write.\
The simplest and most straight-forward one: you can't implement the same trait twice,
in a crate, for the same type.

For example:

> 記述できるトレイトの実装には制限があります。
> 最も単純で最も簡単な1つは、クレート内で任意の型に同じトレイトを2回実装できないことです。
>
> 例えば・・・

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

The compiler will reject it:

> コンパイラーはそれを拒否します。

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

There can be no ambiguity as to what trait implementation should be used when `IsEven::is_even`
is invoked on a `u32` value, therefore there can only be one.

> `u32`値に対して`IsEven::is_even`が呼び出されたとき、どのトレイトの実装を仕様するべきかについて曖昧さがあってはなりません。
> 従って、1つしか存在できません。

## Orphan rule（孤立ルール）

Things get more nuanced when multiple crates are involved.
In particular, at least one of the following must be true:

- The trait is defined in the current crate
- The implementor type is defined in the current crate

> 複数のクレートが巻き込まれると、モノはより微妙な差異がでてきます。
> 特に、次の内、少なくとも1つは成立しなくてはなりません。
>
> - トレイトが、現在のクレートで定義されている。
> - 実装型（上記トレイトを実装する型）が、現在のクレートで定義されている。

This is known as Rust's **orphan rule**. Its goal is to make the method resolution
process unambiguous.

> これはRustの**孤立ルール**として知られています。
> その目的は、メソッド解決処理を曖昧ににしないことです。

Imagine the following situation:

- Crate `A` defines the `IsEven` trait
- Crate `B` implements `IsEven` for `u32`
- Crate `C` provides a (different) implementation of the `IsEven` trait for `u32`
- Crate `D` depends on both `B` and `C` and calls `1.is_even()`

> 次の状況を想像してください。
>
> - クレート`A`が、`IsEven`トレイトを定義する。
> - クレート`B`が、`u32`に対して`IsEven`を実装する。
> - クレート`C`が、`u32`に対して、`IsEven`の異なる実装を提供する。
> - クレート`D`は、`B`と`C`の両方に依存しており、`1.is_even()`を呼び出す。

Which implementation should be used? The one defined in `B`? Or the one defined in `C`?\
There's no good answer, therefore the orphan rule was defined to prevent this scenario.
Thanks to the orphan rule, neither crate `B` nor crate `C` would compile.

> どちらの実装が使用されるべきでしょうか？`B`で定義されたモノでしょうか？それとも`C`で定義されたものでしょうか？
> 良い答えはありません。従って、孤立ルールはこのシナリオを避けるために定義されました。
> 孤立ルールのおかげで、`B`クレートと`C`クレートはどちらもコンパイルされません。

## Further reading（参考資料）

- There are some caveats and exceptions to the orphan rule as stated above.
  Check out [the reference](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)
  if you want to get familiar with its nuances.

> - 上記で述べた孤立ルールには、いくつかの注意事項と例外があります。
>   その微妙な差異に慣れたい場合は、リファレンスを確認してください。
