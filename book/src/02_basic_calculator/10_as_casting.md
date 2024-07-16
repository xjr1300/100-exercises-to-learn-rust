# Conversions, pt. 1（変換、その1）

We've repeated over and over again that Rust won't perform
implicit type conversions for integers.\
How do you perform _explicit_ conversions then?

> Rustは整数の暗黙的な型変換を行わないことを何度も繰り返し説明しました。
> では、どの用に*明示的に*変換を行うのでしょうか？

## `as`

You can use the `as` operator to convert between integer types.\
`as` conversions are **infallible**.
For example:

> 整数型間を変換するために`as`演算子を使用できます。
> `as`変換は**失敗しません**。
> 例えば・・・

```rust
let a: u32 = 10;

// Cast `a` into the `u64` type
// `a`を`u64`型にキャストします。
let b = a as u64;

// You can use `_` as the target type
// if it can be correctly inferred
// by the compiler. For example:
// コンパイラによって正確に推論される場合、目的の型として`_`を使用できます。
// 例えば・・・
let c: u64 = a as _;
```

The semantics of this conversion are what you expect: all `u32` values are valid `u64`
values.

> この変換の意味は、何を期待するかを示します。
> すべての`u32`値は有効な`u64`値です。

### Truncation（切り捨て）

Things get more interesting if we go in the opposite direction:

> 反対方向に行った場合、より興味深いことが起こります。

```rust
// A number that's too big
// to fit into a `u8`
// `u8`に収まらないほど大きな数です。
let a: u16 = 255 + 1;
let b = a as u8;
```

This program will run without issues, because `as` conversions are infallible.
But what is the value of `b`?
When going from a larger integer type to a smaller, the Rust compiler will perform
a **truncation**.

> `as`変換は失敗しないため、このプログラムは問題なく実行されます。
> では、`b`の値は何でしょうか？
> 大きな整数型を小さな整数型に変換したとき、Rustコンパイラーは**切り捨て**を行います。

To understand what happens, let's start by looking at how `256u16` is
represented in memory, as a sequence of bits:

> 何が起こるかを理解するために、`256u16`がビットシーケンスとしてメモリ内で表現される方法を確認することから始めましょう。

```text
 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0
|               |               |
+---------------+---------------+
  First 8 bits    Last 8 bits
```

When converting to a `u8`, the Rust compiler will keep the last 8 bits of a `u16`
memory representation:

> `u8`に変換するとき、Rustコンパイラーは`u16`のメモリ表現の最後の8ビットを維持します。

```text
 0 0 0 0 0 0 0 0
|               |
+---------------+
  Last 8 bits
```

Hence `256 as u8` is equal to `0`. That's... not ideal, in most scenarios.\
In fact, the Rust compiler will actively try to stop you if it sees you trying
to cast a literal value which will result in a truncation:

> よって、`256 as u8`は`0`と等しいです。ほとんどのシナリオで、それは・・・理想的ではありません。
> 実際に、Rustコンパイラーは、切り捨ての結果になるリテラル値をキャストする試みを確認すると、積極的に停止しようとします。

```text
error: literal out of range for `i8`
  |
4 |     let a = 255 as i8;
  |             ^^^
  |
  = note: the literal `255` does not fit into the type `i8` whose range is `-128..=127`
  = help: consider using the type `u8` instead
  = note: `#[deny(overflowing_literals)]` on by default
```

### Recommendation（推奨事項）

As a rule of thumb, be quite careful with `as` casting.\
Use it _exclusively_ for going from a smaller type to a larger type.
To convert from a larger to smaller integer type, rely on the
[_fallible_ conversion machinery](../05_ticket_v2/13_try_from.md) that we'll
explore later in the course.

> 経験則として、`as`キャストの使用はとても注意して行ってください。
> それは、小さな型から大きな型に変換するために*排他的に*使用してください。
> *失敗しない変換機械*についてコースの後半で探求します。

### Limitations（制限事項）

Surprising behaviour is not the only downside of `as` casting.
It is also fairly limited: you can only rely on `as` casting
for primitive types and a few other special cases.\
When working with composite types, you'll have to rely on
different conversion mechanisms ([fallible](../05_ticket_v2/13_try_from.md)
and [infallible](../04_traits/09_from.md)), which we'll explore later on.

> 驚くべき振る舞いは、`as`キャストの唯一の欠点ではありません。
> また、それはかなり制限されています。
> `as`キャストは、プリミティブな型といくつかの他の特別な場面にのみ依存しています。
> 複合型で作業しているとき、後で探求する*失敗する*と*失敗しない*の異なる変換メカニズムに依存しなければなりません。

## Further reading（参考資料）

- Check out [Rust's official reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast)
  to learn the precise behaviour of `as` casting for each source/target combination,
  as well as the exhaustive list of allowed conversions.
  変換を許可する網羅的なリストと同様に、それそれのソース/ターゲットの組み合わせに対する`as`キャストの正確な動作を学ぶために、*Rustの公式リファレンス*を参照してください。
