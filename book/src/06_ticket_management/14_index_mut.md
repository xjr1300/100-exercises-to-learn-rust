# Mutable indexing（可変インデックス）

`Index` allows read-only access. It doesn't let you mutate the value you
retrieved.

> `Index`は読み込み専用アクセスを許可します。それは取得した値を変更させません。

## `IndexMut`

If you want to allow mutability, you need to implement the `IndexMut` trait.

> 変異性を許可したい場合、`IndexMut`トレイトを実装する必要があります。

```rust
// Slightly simplified
pub trait IndexMut<Idx>: Index<Idx>
{
    // Required method
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` can only be implemented if the type already implements `Index`,
since it unlocks an _additional_ capability.

> `IndexMut`は、それが _追加_ 能力をアンロックするため、型がすでに`Index`を実装している場合にのみ実装できます。
