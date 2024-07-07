# `From` and `Int`（FromとInto）

Let's go back to where our string journey started:

> 文字列の旅を始めたことろに戻りましょう。

```rust
let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
```

We now know enough to start unpacking what `.into()` is doing here.

> ここで、`.into()`が何をしているのかを解明を始める十分な理解があります。

## The problem（課題）

This is the signature of the `new` method:

> これは、`new`meソッドのシグネチャーです。

```rust
impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Self {
        // [...]
    }
}
```

We've also seen that string literals (such as `"A title"`) are of type `&str`.\
We have a type mismatch here: a `String` is expected, but we have a `&str`.
No magical coercion will come to save us this time; we need **to perform a conversion**.

また、`"A title"`のような文字列リテラルが`&str`型であることを確認しました。
ここで、型のミスマッチがあります。`String`が期待されていますが、`&str`があります。
今回は、魔法の強制はこれを救ってはくれません。**変換を実行する**必要があります。

## `From` and `Into`（FromとInto）

The Rust standard library defines two traits for **infallible conversions**: `From` and `Into`,
in the `std::convert` module.

> Rust標準ライブラリは、**失敗しない変換**のための2つのトレイトである`From`と`Into`を`std::convert`モジュールで定義しています。

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

These trait definitions showcase a few concepts that we haven't seen before: **supertraits** and **implicit trait bounds**.
Let's unpack those first.

> これらのトレイトの定義は、前に確認したいくつかの概念を披露しています。**スーパートレイト**と**暗黙的なトレイト制約**です。
> まず、それらを解明しましょう。

### Supertrait / Subtrait（スーパートレイト / サブトレイト）

The `From: Sized` syntax implies that `From` is a **subtrait** of `Sized`: any type that
implements `From` must also implement `Sized`.
Alternatively, you could say that `Sized` is a **supertrait** of `From`.

> `From: Sized`構文は、`From`が`Sized`のサブトレイトであることを暗に意味しています。
> `From`を実装する任意の型は、`Sized`も実装しなければなりません。
> 代わりに、`Sized`は`From`のスーパートレイトであると言うことができます。

### Implicit trait bounds（暗黙的なトレイト制約）

Every time you have a generic type parameter, the compiler implicitly assumes that it's `Sized`.

For example:

> ジェネリックな型パラメーターがあるときはいつでも、コンパイラーは暗黙的にそれが`Sized`であることを仮定します。
>
> 例えば・・・

```rust
pub struct Foo<T> {
    inner: T,
}
```

is actually equivalent to:

> 上記は実際に次と等価です。

```rust
pub struct Foo<T: Sized>
{
    inner: T,
}
```

In the case of `From<T>`, the trait definition is equivalent to:

> `From<T>`の場合、トレイトの定義は次と等価です。

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

In other words, _both_ `T` and the type implementing `From<T>` must be `Sized`, even
though the former bound is implicit.

> 言い換えると、`T`と　`From<T>`を実装した型は両方とも`Sized`でなければならず、前者の制約は暗黙的です。

### Negative trait bounds（ネガティブトレイト制約）

You can opt out of the implicit `Sized` bound with a **negative trait bound**:

> **ネガティブトレイト制約**を使用して、暗黙的な`Sized`制約を外すことができます。

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            This is a negative trait bound
    //            これはネガティブトレイト制約です。
    inner: T,
}
```

This syntax reads as "`T` may or may not be `Sized`", and it allows you to
bind `T` to a DST (e.g. `Foo<str>`). It is a special case, though: negative trait bounds are exclusive to `Sized`,
you can't use them with other traits.

> この構文は、「`T`は`Sized`かもしれないし、そうでないかもしれない」と読みとれ、例えば`Foo<str>`のように、それは`T`をDSTに拘束できるようにしまう。

## `&str` to `String`（&strからStringへ）

In [`std`'s documentation](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)
you can see which `std` types implement the `From` trait.\
You'll find that `String` implements `From<&str> for String`. Thus, we can write:

> `std`のドキュメント内で、どの`std`の方が`From`トレイトを実装しているか確認できます。
> `String`が`From<&str>`を実装していることを確認できます。よって、次のように記述できました。

```rust
let title = String::from("A title");
```

We've been primarily using `.into()`, though.\
If you check out the [implementors of `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)
you won't find `Into<&str> for String`. What's going on?

> しかし、主に`.into()`を使用してきました。
> `Into`の実装を確認した場合、`Into<&str> for String`は見つかりません。何が起こっているのでしょうか？

`From` and `Into` are **dual traits**.\
In particular, `Into` is implemented for any type that implements `From` using a **blanket implementation**:

> `From`と`Into`は**デュアルトレイト**です。
> 特に、`Into`は、**ブランケット実装**を使用して、`From`を実装する任意の型に対して実装されます。

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

If a type `U` implements `From<T>`, then `Into<U> for T` is automatically implemented. That's why
we can write `let title = "A title".into();`.

> 型`U`が`From<T>`を実装している場合、`Into<U> for T`は自動的に実装されます。
> これが、`let title = "A title".into();`と記述できた理由です。

## `.into()`

Every time you see `.into()`, you're witnessing a conversion between types.\
What's the target type, though?

> `.into()`を確認するたびに、型間の変換を目撃します。
> しかし、目的の型はなんでしょうか？

In most cases, the target type is either:

- Specified by the signature of a function/method (e.g. `Ticket::new` in our example above)
- Specified in the variable declaration with a type annotation (e.g. `let title: String = "A title".into();`)

> ほとんどの場合、目的の方は次のいずれかです。
>
> - 関数/メソッドのシグネチャーによって指定される（例えば、上記列の`Ticket::new`）
> - 型注釈を持つ変数宣言で指定される（例えば、`let title: String = "A title".into();`）

`.into()` will work out of the box as long as the compiler can infer the target type from the context without ambiguity.

> `.into()`は、コンパイラーが曖昧なくコンテキストから目的の型を推測できる限り、そのまま機能します。
