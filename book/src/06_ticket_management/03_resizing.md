# Resizing（リサイズ）

We said that `Vec` is a "growable" vector type, but what does that mean?
What happens if you try to insert an element into a `Vec` that's already at maximum capacity?

> `Vec`が「成長可能な」ベクター型であると言いましたが、それは何を意味するのでしょうか？
> すでに最大容量になっている`Vec`に要素を挿入することを試みた場合、何が起こるのでしょうか？

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
numbers.push(3); // Max capacity reached（最大容量に達した）
numbers.push(4); // What happens here?（ここで何が起こるのか？）
```

The `Vec` will **resize** itself.\
It will ask the allocator for a new (larger) chunk of heap memory, copy the elements over, and deallocate the old memory.

> `Vec`は自分自身で**リサイズ**します。
> ヒープメモリの新しく大きな塊をアロケーターに要求して、要素をコピーして、古いメモリを解放します。

This operation can be expensive, as it involves a new memory allocation and copying all existing elements.

> 新しい割り当てと、既存の要素のすべてのコピーを巻き込むため、この操作はコストが高くなる可能性があります。

## `Vec::with_capacity`

If you have a rough idea of how many elements you'll store in a `Vec`, you can use the `Vec::with_capacity`
method to pre-allocate enough memory upfront.\
This can avoid a new allocation when the `Vec` grows, but it may waste memory if you overestimate actual usage.

> `Vec`に保存する要素のおおよその数がわかっている場合、前もって十分なメモリを事前割り当てするために`Vec::with_capacity`メソッドを使用できます。
> これは、`Vec`が成長したとき新しい割り当てを避けることができますが、実際の使用量を過大評価した場合、メモリを浪費するかもしれません。

Evaluate on a case-by-case basis.

> ケースによって評価することが基本です。
