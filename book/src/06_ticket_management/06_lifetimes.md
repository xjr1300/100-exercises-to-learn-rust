# Lifetimes（ライブタイム）

Let's try to complete the previous exercise by adding an implementation of `IntoIterator` for `&TicketStore`, for
maximum convenience in `for` loops.

> `for`ループを最大限に便利にするために、`&TicketStore`に対して`IntoIterator`の実装を追加して、前の演習を完成することを試みましょう。

Let's start by filling in the most "obvious" parts of the implementation:

> 実装の最も「明確な」部分を埋めることから始めましょう。

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // What goes here?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

What should `type IntoIter` be set to?\
Intuitively, it should be the type returned by `self.tickets.iter()`, i.e. the type returned by `Vec::iter()`.\
If you check the standard library documentation, you'll find that `Vec::iter()` returns an `std::slice::Iter`.
The definition of `Iter` is:

> `type IntoIter`は何を設定するべきでしょうか？
> 直感的に、それは、`self.tickets.iter()`によって返される型、例えば`Vec::iter()`によって返される型であるべきです。
> 標準ライブラリのドキュメントを確認した場合、`Vec::iter()`は`std::slice::Iter`を返すことを見つけるはずです。
> `Iter`の定義は・・・

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` is a **lifetime parameter**.

> `'a`は**ライフタイムパラメーター**です。

## Lifetime parameters（ライフタイムパラメーター）

Lifetimes are **labels** used by the Rust compiler to keep track of how long a reference (either mutable or
immutable) is valid.\
The lifetime of a reference is constrained by the scope of the value it refers to. Rust always makes sure, at compile-time,
that references are not used after the value they refer to has been dropped, to avoid dangling pointers and use-after-free bugs.

> ライフタイムは、可変または不変参照が有効な長さを追跡するためにRustコンパイラーによって使用されます。
> 参照のライフタイムは、それが参照する値のスコープによって制限されます。
> Rustは、ダングリングポインターと解放された後に使用するバグを避けるために、コンパイル時に、参照している値がドロップされた後で、参照が使用されないことを常に確認します。

This should sound familiar: we've already seen these concepts in action when we discussed ownership and borrowing.
Lifetimes are just a way to **name** how long a specific reference is valid.

> これは、おなじみのはずです。所有権と借用を議論した後に、実際にこれらの概念をすでに確認しました。
> ライフタイムは、特定の参照が有効な期間を**名前付け**する単なる方法です。

Naming becomes important when you have multiple references and you need to clarify how they **relate to each other**.
Let's look at the signature of `Vec::iter()`:

> 複数の参照があり、それらがどのように**お互いに関連している**かを明確にする必要があるとき、名前付けは重要になります。

```rust
impl <T> Vec<T> {
    // Slightly simplified
    // 少し簡略化してあります。
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` is generic over a lifetime parameter, named `'a`.\
`'a` is used to **tie together** the lifetime of the `Vec` and the lifetime of the `Iter` returned by `iter()`.
In plain English: the `Iter` returned by `iter()` cannot outlive the `Vec` reference (`&self`) it was created from.

> `Vec::iter()`は、`'a`という名前が付けられた、ジェネリックなライフタイムパラメーターです。
> `'a`は、`Vec`のライフタイムと`iter()`によって返される`Iter`のライフタイムを**一緒に縛り付ける**ために使用されます。
> 簡単な英語で言えば、`Iter()`によって返された`Iter`は、それが作成された`Vec`の参照（`&self`）よりも長生きできません。

This is important because `Vec::iter`, as we discussed, returns an iterator over **references** to the `Vec`'s elements.
If the `Vec` is dropped, the references returned by the iterator would be invalid. Rust must make sure this doesn't happen,
and lifetimes are the tool it uses to enforce this rule.

> 議論したように`Vec::iter`は、`Vec`の要素への参照を返すイテレーターを返すため重要です。
> `Vec`がドロップされた場合、そのイテレーターによって返された参照は無効になります。
> Rustは、これが発生しないことを確実にしなければならず、ライフタイムはこのルールを強制するために使用されるツールです。

## Lifetime elision（ライフタイムの省略）

Rust has a set of rules, called **lifetime elision rules**, that allow you to omit explicit lifetime annotations in many cases.
For example, `Vec::iter`'s definition looks like this in `std`'s source code:

> Rustは**ライフタイムの省略ルール**と呼ばれるルールの集合を持っており、多くの場合で明示的なライフタイム注釈を省略することができます。
> 例えば、`Vec::iter`の定義は、`std`のソースコードでは次のようになります。

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

No explicit lifetime parameter is present in the signature of `Vec::iter()`.
Elision rules imply that the lifetime of the `Iter` returned by `iter()` is tied to the lifetime of the `&self` reference.
You can think of `'_` as a **placeholder** for the lifetime of the `&self` reference.

> `Vec::iter()`のシグネチャーに、明示的なライフタイムパラメーターは存在しません。
> 省略ルールは、`iter()`によって返された`Iter`のライフタイムは、`&self`参照のライフタイムに縛り付けられていることを暗示します。
> `'_`を`&self`参照のライフタイムの**プレースホルダー**として考えれます。

See the [References](#references) section for a link to the official documentation on lifetime elision.\
In most cases, you can rely on the compiler telling you when you need to add explicit lifetime annotations.

> ライフタイムの省略に関する公式ドキュメントへのリンクについては、参照節を参照してください。
> ほとんどの場合、明示的なライフタイム注釈を追加する必要があるときは、コンパイラーからの通知に頼れます。

## References

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
