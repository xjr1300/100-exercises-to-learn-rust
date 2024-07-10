# `.iter()`

`IntoIterator` **consumes** `self` to create an iterator.

> `IntoIterator`は、イテレーターを作成するために`self`を**消費**します。

This has its benefits: you get **owned** values from the iterator.
For example: if you call `.into_iter()` on a `Vec<Ticket>` you'll get an iterator that returns `Ticket` values.

> これには利点があります。イテレーターから値を**所有**できることです。
> 例えば、`Vec<Ticket>`に`.into_iter()`を呼び出した場合、`Ticket`値を返すイテレーターを得られます。

That's also its downside: you can no longer use the original collection after calling `.into_iter()` on it.
Quite often you want to iterate over a collection without consuming it, looking at **references** to the values instead.
In the case of `Vec<Ticket>`, you'd want to iterate over `&Ticket` values.

> またこれには欠点があります。それに`.into_iter()`を呼び出した後、もはやオリジナルのコレクションを使用できなくなることです。
> かなりの頻度で、コレクションを消費することなく、代わりに値の**参照**を確認するために、コレクションを反復操作することを望みます。
> `Vec<Ticket>`の場合、`&Ticket`値を反復操作することを望みます。

Most collections expose a method called `.iter()` that returns an iterator over references to the collection's elements.
For example:

> ほとんどのコレクションは、コレクションの要素への参照を返すイテレーターを返す`.iter()`と呼ばれるメソッドを公開しています。
> 例えば・・・

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
for n in numbers.iter() {
    // [...]
}
```

This pattern can be simplified by implementing `IntoIterator` for a **reference to the collection**.
In our example above, that would be `&Vec<Ticket>`.\
The standard library does this, that's why the following code works:

> このパターンは、**コレクションへの参照**に対して、`IntoIterator`を実装することで簡略化できます。
> 上記例において、それは`&Vec<Ticket>`です。
> 標準ライブラリはこれを行うため、次のコードが機能する理由になります。

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
// We didn't have to call `.iter()` explicitly
// It was enough to use `&numbers` in the `for` loop
// ここで`n`は`&u32`型です。
// 明示的に`.iter()`を呼び出す必要はありません。
// それは`for`ループで`&numbers`を使用するだけで十分でした。
for n in &numbers {
    // [...]
}
```

It's idiomatic to provide both options:

- An implementation of `IntoIterator` for a reference to the collection.
- An `.iter()` method that returns an iterator over references to the collection's elements.

> それは次の両方の選択肢を提供する慣用句です。
>
> - コレクションへの参照に対する`IntoIterator`の実装
> - これクソんの要素への参照を返すイテレーターを返す`.iter()`メソッド

The former is convenient in `for` loops, the latter is more explicit and can be used in other contexts.

前者は`for`ループで便利で、後者はより明示的で他のコンテキストで使用されます。
