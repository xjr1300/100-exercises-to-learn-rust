# Intro

In the previous chapter we modelled `Ticket` in a vacuum: we defined its fields and their constraints, we learned
how to best represent them in Rust, but we didn't consider how `Ticket` fits into a larger system.
We'll use this chapter to build a simple workflow around `Ticket`, introducing a (rudimentary) management system to
store and retrieve tickets.

> 前の章において、世間から孤立して`Ticket`をモデリングしました。
> そのフィールドとそれらの制約を定義して、Rustでそれらを表現する最良の方法を学びましたが、大きなシステムに`Ticket`を適用する方法を考えていません。
> チケットを保存、そして取得するための基本的な管理システムを導入して、`Ticket`に関する単純なワークフローを構築するためにこの章を使用します。

The task will give us an opportunity to explore new Rust concepts, such as:

- Stack-allocated arrays
- `Vec`, a growable array type, and slices
- `Iterator` and `IntoIterator`, for iterating over collections
- Slices (`&[T]`), to work with parts of a collection
- Lifetimes, to describe how long references are valid
- `HashMap` and `BTreeMap`, two key-value data structures
- `Eq` and `Hash`, to compare keys in a `HashMap`
- `Ord` and `PartialOrd`, to work with a `BTreeMap`
- `Index` and `IndexMut`, to access elements in a collection

> そのタスクは、次のような新しいRustの概念を探求する機会を与えてくれます。
>
> - スタックに割り当てられた配列
> - 成長可能な配列型である`Vec`とスライス
> - コレクションを反復処理する`Iterator`と`IntoIterator`
> - コレクションの一部で動作するスライス（`&[T]`）
> - 参照が有効な期間を説明するライフタイム
> - キーと値のデータ構造である2つの`HashMap`と`BTreeMap`
> - `HashMap`内のキーを比較する`Eq`と`Hash`
> - `BTreeMap`で必要な`Ord`と`PartialOrd`
> - コレクション内の要素にアクセスする`Index`と`IndexMut`
