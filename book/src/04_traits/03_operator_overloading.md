# Operator overloading（演算子のオーバーロード）

Now that we have a basic understanding of what traits are, let's circle back to **operator overloading**.
Operator overloading is the ability to define custom behavior for operators like `+`, `-`, `*`, `/`, `==`, `!=`, etc.

> 現在、トレイトが何か基本的な理解を得たので、**演算子のオーバーロード**に戻りましょう。
> 演算子のオーバーロードは、`+`、`-`、`*`、`/`、`==`、`!=`などの演算子のカスタムな振る舞いを定義する能力です。

## Operators are traits（演算子はトレイト）

In Rust, operators are traits.\
For each operator, there is a corresponding trait that defines the behavior of that operator.
By implementing that trait for your type, you **unlock** the usage of the corresponding operators.

> Rustにおいて、演算子はトレイトです。
> それぞれの演算子には、その演算子の振る舞いを定義するために、対応したトレイトがあります。
> 型にそのトレイトを実装することで、対応する演算子の使用を**解除**します。

For example, the [`PartialEq` trait](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) defines the behavior of
the `==` and `!=` operators:

> 例えば、`PartialEq`トレイトは、`==`と`!=`演算子の振る舞いを定義します。

```rust
// The `PartialEq` trait definition, from Rust's standard library
// (It is *slightly* simplified, for now)
// Rust標準ライブラリの`PartialEq`トレイトの定義です。
// （ここでは、それは*すこし*簡略化されています）
pub trait PartialEq {
    // Required method
    // 要求されるメソッドです。
    //
    // `Self` is a Rust keyword that stands for
    // "the type that is implementing the trait"
    // `Self`は「トレイトを実装した型」を表すRustのキーワードです。
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    // 提供されたメソッドです。
    fn ne(&self, other: &Self) -> bool { ... }
}
```

When you write `x == y` the compiler will look for an implementation of the `PartialEq` trait for the types of `x` and `y`
and replace `x == y` with `x.eq(y)`. It's syntactic sugar!

> `x == y`を記述すると、`x`と`y`の型の`PartialEq`トレイトの実装を探し、`x == y`を`x.eq(y)`に置き換えます。
> それは糖衣構文です！

This is the correspondence for the main operators:

> 次は主要な演算子の対応です。

| Operator                 | Trait                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` and `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=`, and `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Arithmetic operators live in the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module,
while comparison ones live in the [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) module.

> 算術演算子は、`std::ops`モジュールにあり、比較演算子は`std::cmp`モジュールにあります。

## Default implementations（デフォルト実装）

The comment on `PartialEq::ne` states that "`ne` is a provided method".\
It means that `PartialEq` provides a **default implementation** for `ne` in the trait definition—the `{ ... }` elided
block in the definition snippet.\
If we expand the elided block, it looks like this:

> `PartialEq::ne`のコメントは、「`ne`は提供されたメソッドです」と述べています。
> それは、`PartialEq`がトレイト定義内で`ne`の**デフォルト実装**を提供していることを意味します。
> それは、定義スニペット内の省略されたブロック`{ ... }`です。
> 省略されたブロックを展開すると、次が表示されます。

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

It's what you expect: `ne` is the negation of `eq`.\
Since a default implementation is provided, you can skip implementing `ne` when you implement `PartialEq` for your type.
It's enough to implement `eq`:

> それは期待したものです。`ne`は`eq`の否定です。
> デフォルト実装が提供されているため、型に`PartialEq`を実装する時、`ne`の実装をスキップできます。
> `eq`の実装で十分です。

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }

    // No `ne` implementation here
    // ここに`ne`の実装はありません。
}
```

You are not forced to use the default implementation though.
You can choose to override it when you implement the trait:

> しかし、デフォルト実装を使用することを強制されません。
> トレイを実装するとき、上書きすることを選択できます。

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Custom implementation
        // 独自の実装
    }

    fn ne(&self, other: &MyType) -> bool {
        // Custom implementation
        // 独自の実装
    }
}
```
