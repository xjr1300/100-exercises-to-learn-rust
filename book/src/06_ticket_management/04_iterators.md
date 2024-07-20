# Iteration（反復操作）

During the very first exercises, you learned that Rust lets you iterate over collections using `for` loops.
We were looking at ranges at that point (e.g. `0..5`), but the same holds true for collections like arrays and vectors.

> とても前の方の演習の間、Rustが`for`ループを使用してコレクションを反復操作させることを学びました。
> その時、例えば`0..5`のような範囲を確認しましたが、配列やベクターのようなコレクションについても同じことが当てはまります。

```rust
// It works for `Vec`s
// それは`Vec`で機能します。
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// It also works for arrays
// それは配列でも機能します。
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

It's time to understand how this works under the hood.

> 現在、内部でこれが機能する方法を理解する時間です。

## `for` desugaring（forの脱糖）

Every time you write a `for` loop in Rust, the compiler _desugars_ it into the following code:

> Rustで`for`ループを記述するたびに、コンパイラーはそれを次のコードに「脱糖」します。

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` is another looping construct, on top of `for` and `while`.\
A `loop` block will run forever, unless you explicitly `break` out of it.

> `loop`は`for`と`while`の上にある別のループ構造です。
> `loop`ブロックは、それを明示的に`break`するまで、永遠に実行されます。

## `Iterator` trait（Iteratorトレイト）

The `next` method in the previous code snippet comes from the `Iterator` trait.
The `Iterator` trait is defined in Rust's standard library and provides a shared interface for
types that can produce a sequence of values:

> 前のコードスニペットの`next`メソッドは`Iterator`トレイトから由来します。
> `Iterator`トレイトはRust標準ライブラリに定義され、値のシーケンスを生成できる型に対して共有されたインターフェイスを提供します。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type specifies the type of the values produced by the iterator.

> `Item`関連型は、イテレーターによって生成される値の型を指定します。

`next` returns the next value in the sequence.\
It returns `Some(value)` if there's a value to return, and `None` when there isn't.

> `next`はシーケンスの次の値を返します。
> 値が存在する場合にそれは`Some(value)`を返し、存在しない場合は`None`を返します。

Be careful: there is no guarantee that an iterator is exhausted when it returns `None`. That's only
guaranteed if the iterator implements the (more restrictive)
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) trait.

> イテレーターが`None`を返したとき、イテレーターが使い果たされたことを保証しないことに注意してください。
> それは、イテレーターがより制限された`FusedIterator`トレイトを実装している場合のみ保証されます。

## `IntoIterator` trait（IntoIteratorトレイト）

Not all types implement `Iterator`, but many can be converted into a type that does.\
That's where the `IntoIterator` trait comes in:

> すべての型が`Iterator`を実装しているわけではありませんが、多くはその型に変換できます。
> ここで`IntoIterator`トレイトが登場します。

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

The `into_iter` method consumes the original value and returns an iterator over its elements.\
A type can only have one implementation of `IntoIterator`: there can be no ambiguity as to what `for` should desugar to.

> `into_iter`メソッドは、オリジナルの値を消費して、その要素のイテレーターを返します。
> 型は`IntoIterator`の実装を1つだけ持つことができ、`for`がどのように脱糖されるべきかについての曖昧さはありません。

One detail: every type that implements `Iterator` automatically implements `IntoIterator` as well.
They just return themselves from `into_iter`!

> 1つの詳細: `Iterator`を実装するすべての型は、自動的に`IntoIterator`も同様に実装します。
> それらは単に、`into_iter`から返されたそれら自身を返します。

## Bounds checks（境界チェック）

Iterating over iterators has a nice side effect: you can't go out of bounds, by design.\
This allows Rust to remove bounds checks from the generated machine code, making iteration faster.

> イテレーターの反復操作は、良い副作用があります。設計上、境界を超えることはできません。
> これは、Rustに生成された機械語から境界チェックを削除することを許可するため、反復操作を高速にします。

In other words,

> 言い換えれば・・・

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

is usually faster than

> 通常、上記は次よりも早いです。

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

There are exceptions to this rule: the compiler can sometimes prove that you're not going out of bounds even
with manual indexing, thus removing the bounds checks anyway. But in general, prefer iteration to indexing
where possible.

> このルールには例外があります。
> 時々、コンパイラーは、手動でインデックスで指定しても境界を越えていないことを証明できるため、その場合は境界チェックが削除されます。
> ただし、可能であれば、インデックスよりも反復処理を優先してください。
