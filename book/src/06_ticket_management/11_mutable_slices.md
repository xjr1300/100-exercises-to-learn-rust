# Mutable slices（可変スライス）

Every time we've talked about slice types (like `str` and `[T]`), we've used their immutable borrow form (`&str` and `&[T]`).\
But slices can also be mutable!

> `str`と`[T]`のしょうなスライス型について話すたびに、`&str`と`&[T]`のようなそれらの不変借用を使用してきました。

Here's how you create a mutable slice:

> ここに、可変スライスを作成する方法を示します。

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

You can then modify the elements in the slice:

> その後、スライス内の要素を修正できます。

```rust
slice[0] = 42;
```

This will change the first element of the `Vec` to `42`.

> これは、`Vec`の先頭の要素を`42`に変更します。

## Limitations（制限事項）

When working with immutable borrows, the recommendation was clear: prefer slice references over references to
the owned type (e.g. `&[T]` over `&Vec<T>`).\
That's **not** the case with mutable borrows.

> 不変借用で作業するとき、推奨は明確でした。
> 例えば、`&Vec<T>`よりも`&[T]`のように、所有された型の参照よりも、スライス参照を選択することです。
> しかし、可変借用の場合は**そうではありません**。

Consider this scenario:

> 次のシナリオを考えてください。

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

It won't compile!\
`push` is a method on `Vec`, not on slices. This is the manifestation of a more general principle: Rust won't
allow you to add or remove elements from a slice. You will only be able to modify/replace the elements that are
already there.

> これはコンパイルされません。
> `push`は`Vec`のメソッドで、スライスではありません。これはより一般的な原則の現れです。
> Rustは、スライスから要素を追加または削除させません。
> （可変スライスは、）すでに存在する要素のみを修正／置き換えできます。

In this regard, a `&mut Vec` or a `&mut String` are strictly more powerful than a `&mut [T]` or a `&mut str`.\
Choose the type that best fits based on the operations you need to perform.

> この観点から、`&mut Vec`または`&mut String`は、`&mut [T]`または`&mut str`よりも非常に強力です。
> 実行する必要がある操作に基づいて、最適な型を選択してください。
