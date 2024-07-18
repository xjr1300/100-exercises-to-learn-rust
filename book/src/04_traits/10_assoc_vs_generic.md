# Generics and associated types（ジェネリックと関連型）

Let's re-examine the definition for two of the traits we studied so far, `From` and `Deref`:

> これまでに学んだ`From`と`Deref`の2つのトレイトの定義を再調査しましょう。

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;

    fn deref(&self) -> &Self::Target;
}
```

They both feature type parameters.\
In the case of `From`, it's a generic parameter, `T`.\
In the case of `Deref`, it's an associated type, `Target`.

> それら両方は、型パラメーターが特徴です。
> `From`の場合、`T`はジェネリックパラメーターです。
> `Deref`の場合、`Target`は関連型です。

What's the difference? Why use one over the other?

> 違いは何でしょうか？なぜどちらかを使うのでしょうか？

## At most one implementation（最大1つの実装）

Due to how deref coercion works, there can only be one "target" type for a given type. E.g. `String` can
only deref to `str`.
It's about avoiding ambiguity: if you could implement `Deref` multiple times for a type,
which `Target` type should the compiler choose when you call a `&self` method?

> 参照外し型強制が機能する仕組みにより、特定の型に対する1つの「目的」型が存在します。
> 例えば、`String`は、`str`にのみ参照外しされます。
> それは曖昧さを避けるためです。ある型に対して複数回`Deref`を実装した場合、`&self`メソッドを呼び出した時、コンパイラーはどの`Target`型を選択するべきでしょうか？

That's why `Deref` uses an associated type, `Target`.\
An associated type is uniquely determined **by the trait implementation**.
Since you can't implement `Deref` more than once, you'll only be able to specify one `Target` for a given type
and there won't be any ambiguity.

> それが、`Deref`が関連型`Target`を使用する理由です。
> 関連型は、**トレイトの実装によって**一意に決定されます。
> `Deref`を1つ以上実装できないため、特定の型に対してたった1つの`Target`を指定することができ、曖昧さはありません。

## Generic traits（ジェネリックトレイト）

On the other hand, you can implement `From` multiple times for a type, **as long as the input type `T` is different**.
For example, you can implement `From` for `WrappingU32` using both `u32` and `u16` as input types:

> 一方、任意の型に対して、**入力型`T`が異なる限り**、複数回`From`を実装できます。
> 例えば、`u32`と`u16`の両方を入力型として使用して、`WrappingU321に対して`From`を実装できます。

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

This works because `From<u16>` and `From<u32>` are considered **different traits**.\
There is no ambiguity: the compiler can determine which implementation to use based on type of the value being converted.

> `From<u16>`と`From<u32>`は**異なるトレイト**とみなされるため、これは機能します。
> 曖昧さはありません。コンパイラーは、値が変換される型に基づいて、どの実装を使用するか決定できます。

## Case study: `Add`（事例研究: Add）

As a closing example, consider the `Add` trait from the standard library:

> 最後の例として、標準ライブラリの`Add`トレイトを考えてみましょう。

```rust
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

It uses both mechanisms:

- it has a generic parameter, `RHS` (right-hand side), which defaults to `Self`
- it has an associated type, `Output`, the type of the result of the addition

> それは両方のメカニズムを使用しています。
>
> - `Self`をデフォルトに持つ`RHS`（右辺）というジェネリックパラメーターがあります。
> - 加算の結果の型である`Output`という関連型があります。

### `RHS`

`RHS` is a generic parameter to allow for different types to be added together.\
For example, you'll find these two implementations in the standard library:

`RHS`は、異なる型を一緒に加算されるようにするためのジェネリックパラメーターです。
例えば、標準ライブラリにこれら2つの実装を見つけるでしょう。

```rust
impl Add<u32> for u32 {
    type Output = u32;

    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // This could be written as `Self::Output` instead.
      // The compiler doesn't care, as long as the type you
      // specify here matches the type you assigned to `Output`
      // right above.
      // これは、代わりに`Self::Output`と記述できます。
      // コンパイラーは、ここで指定した型が、上記`Output`の右に指定されている型と一致している限り気にしません。
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

This allows the following code to compile:

これは、次のコードをコンパイルできるようにします。

```rust
let x = 5u32 + &5u32 + 6u32;
```

because `u32` implements `Add<&u32>` _as well as_ `Add<u32>`.

> なぜなら、`u32`は`Add<u32>`と _同様に_ 、`Add<&u32>`を実装しているからです。

### `Output`

`Output` represents the type of the result of the addition.

> `Output`は、加算の結果の型を表現します。

Why do we need `Output` in the first place? Can't we just use `Self` as output, the type implementing `Add`?
We could, but it would limit the flexibility of the trait. In the standard library, for example, you'll find
this implementation:

> なぜ、そもそも`Output`が必要なのでしょうか？
> 出力として単に`Self`を使用できないのでしょうか？
> できますが、そのトレイトの柔軟性を制限します。
> 例えば、標準ライブラリにおいて、次の実装を見つけるでしょう。

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

The type they're implementing the trait for is `&u32`, but the result of the addition is `u32`.\
It would be impossible[^flexible] to provide this implementation if `add` had to return `Self`, i.e. `&u32` in this case.
`Output` lets `std` decouple the implementor from the return type, thus supporting this case.

> トレイトを実装している型は`&u32`ですが、加算の結果の型は`u32`です。
> `add`が`Self`を返さなければならないとき、例えば`&u32`の場合、この実装を提供することは不可能です。

On the other hand, `Output` can't be a generic parameter. The output type of the operation **must** be uniquely determined
once the types of the operands are known. That's why it's an associated type: for a given combination of implementor
and generic parameters, there is only one `Output` type.

> 一方、`Output`はジェネリックパラメーターにすることはできません。
> 操作の出力型は、一度、オペランドの型が知られると、一意に決定されなければなりません。
> それが、関連型のの理由です。実装者とジェネリックパラメーターの組が与えられると、たった1つの`Output`型しかありません。

## Conclusion（結論）

To recap:

- Use an **associated type** when the type must be uniquely determined for a given trait implementation.
- Use a **generic parameter** when you want to allow multiple implementations of the trait for the same type,
  with different input types.

> 要約します。
>
> - 特定のトレイトの実装のために、型が一意に決定されなければならないとき、**関連型**を使用します。
> - 異なる入力型を持つ同じ型に対して複数のトレイトの実装できるようにしたいとき、**ジェネリックパラメーター**を使用します。

[^flexible]: Flexibility is rarely free: the trait definition is more complex due to `Output`, and implementors have to reason about
what they want to return. The trade-off is only justified if that flexibility is actually needed. Keep that in mind
when designing your own traits.
柔軟性が、コストなしになることはほとんどありません。
トレイトの定義が、`Output`によってより複雑になり、実装者は何を返したいのか理由を考えなければなりません。
トレードオフは、柔軟性が実際に必要な場合にのみ正当化されます。
独自のトレイトを設計するとき、これを覚えておいてください。
