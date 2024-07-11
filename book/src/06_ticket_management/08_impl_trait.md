# `impl Trait`

`TicketStore::to_dos` returns a `Vec<&Ticket>`.\
That signature introduces a new heap allocation every time `to_dos` is called, which may be unnecessary depending
on what the caller needs to do with the result.
It'd be better if `to_dos` returned an iterator instead of a `Vec`, thus empowering the caller to decide whether to
collect the results into a `Vec` or just iterate over them.

> `TicketStore::to_dos`は`Vec<&Ticket>`を返します。
> そのシグネチャーは、`to_dos`が呼ばれるたびに新しいヒープ割り当てを招き、それは呼び出し側が結果で行う必要があることによって、不必要になるかもしれません。
> `to_dos`が`Vec`の代わりにイテレーターを返したほうが良く、それにより呼び出し側が`Vec`に結果を集めるか、それらを単に反復操作するか決定することができます。

That's tricky though!
What's the return type of `to_dos`, as implemented below?

> ただし、それはトリッキーです。
> 次に実装された`to_dos`の戻り値の型は何でしょうか？

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

## Unnameable types（名前を持たない型）

The `filter` method returns an instance of `std::iter::Filter`, which has the following definition:

> `filter`メソッドは`std::iter::Filter`のインスタンスを返し、それは次の定義を持ちます。

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

where `I` is the type of the iterator being filtered on and `P` is the predicate used to filter the elements.\
We know that `I` is `std::slice::Iter<'_, Ticket>` in this case, but what about `P`?\
`P` is a closure, an **anonymous function**. As the name suggests, closures don't have a name,
so we can't write them down in our code.

> `I`はフィルターされるイテレーターの型で、`P`は要素をフィルターする述語です。
> この場合、`I`は`std::slice::Iter<'_, Ticket>`であることを知っていますが、`P`についてはどうでしょうか？

Rust has a solution for this: **impl Trait**.

> Rustはこのための解決策があります。それは**impl Trait**です。

## `impl Trait`

`impl Trait` is a feature that allows you to return a type without specifying its name.
You just declare what trait(s) the type implements, and Rust figures out the rest.

> `impl Trait`は、その名前を指定されずに型を返すことができる機能です。
> 型がどのトレイトを実装しているかだけを宣言して、Rustが残りを見出します。

In this case, we want to return an iterator of references to `Ticket`s:

> この場合、`Ticket`への参照のイテレーターを返したいです。

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

That's it!

> これだけです。

## Generic?

`impl Trait` in return position is **not** a generic parameter.

> `戻り値の位置にある`impl Trait`は、ジェネリックパラメーターでは**ありません**。

Generics are placeholders for types that are filled in by the caller of the function.
A function with a generic parameter is **polymorphic**: it can be called with different types, and the compiler will generate
a different implementation for each type.

> ジェネリックは、関数の呼び出し側によって充填される型に対するプレースホルダーです。
> ジェネリックパラメーターを持つ関数は**多態性**です。
> それは、異なる型で呼び出されることができ、コンパイラーはそれぞれの方に対して異なる実装を生成します。

That's not the case with `impl Trait`.
The return type of a function with `impl Trait` is **fixed** at compile time, and the compiler will generate
a single implementation for it.
This is why `impl Trait` is also called **opaque return type**: the caller doesn't know the exact type of the return value,
only that it implements the specified trait(s). But the compiler knows the exact type, there is no polymorphism involved.

> `impl Trait`の場合はそうではありません。
> `impl Trait`を持つ関数の戻り値の型は、コンパイル時に**固定**され、コンパイラーはそれに対して単一の実装を生成します。
> これが、`impl Trait`が**不透明な戻り値型**とも呼ばれる理由です。
> 呼び出し側は戻り値の正確な型を知りませんが、それが指定されたトレイトを実装していることのみを知っています。
> しかし、コンパイラーは正確な型を知っており、多態性は発生しません。

## RPIT

If you read RFCs or deep-dives about Rust, you might come across the acronym **RPIT**.\
It stands for **"Return Position Impl Trait"** and refers to the use of `impl Trait` in return position.

> Rustに関するRFCまたは深い探求をを読んでいる場合、**RPIT**という頭字語に出くわすかもしれません。
> それは、**「戻り値の位置のImpl Trait」**からなり、戻り値の位置で`impl Trait`を使用することを示しています。
