# References（参照）

What about references, like `&String` or `&mut String`? How are they represented in memory?

> `&String`または`&mut String`のような参照についてはどうでしょうか？
> それらはメモリ内でどのように表現されているのでしょうか？

Most references[^fat] in Rust are represented, in memory, as a pointer to a memory location.\
It follows that their size is the same as the size of a pointer, a `usize`.

> Rustにおけるほとんどの参照は、メモリ内でメモリ位置を指し示すポインターとして表現されています。
> それは、それらのサイズがポインターのサイズと同じで`usize`になります。

You can verify this using `std::mem::size_of`:

> これを`std::mem::size_of`を使用して検証できます。

```rust
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);
```

A `&String`, in particular, is a pointer to the memory location where the `String`'s metadata is stored.\
If you run this snippet:

> 特に、`&String`は、`String`のメタデータが保存されたメモリ位置を指し示すポインターです。
> このスニペットを実行した場合・・・

```rust
let s = String::from("Hey");
let r = &s;
```

you'll get something like this in memory:

> メモリ内でこれは次のようになります。

```text
           --------------------------------------
           |                                    |
      +----v----+--------+----------+      +----|----+
Stack | pointer | length | capacity |      | pointer |
      |  |      |   3    |    5     |      |         |
      +--|  ----+--------+----------+      +---------+
         |          s                           r
         |
         v
       +---+---+---+---+---+
Heap   | H | e | y | ? | ? |
       +---+---+---+---+---+
```

It's a pointer to a pointer to the heap-allocated data, if you will.
The same goes for `&mut String`.

> もし、それをした場合、それはヒープに割り当てられたデータを指し示すポインターのポインターです。
> 同じことは`&mut String`にも当てはまります。

## Not all pointers point to the heap（すべてのポインターがヒープを指し示すわけではない）

The example above should clarify one thing: not all pointers point to the heap.\
They just point to a memory location, which _may_ be on the heap, but doesn't have to be.

> 上記例は、明確に1つのことを説明しています。すべてのポインターがヒープを指し示すわけではありません。
> それらは、単にメモリの位置を指し示しており、それはヒープ上にある_かもしれません_が、そうである必要はありません。

[^fat]: [Later in the course](../04_traits/06_str_slice.md) we'll talk about **fat pointers**,
i.e. pointers with additional metadata. As the name implies, they are larger than
the pointers we discussed in this chapter, also known as **thin pointers**.
コースの後半で、例えば追加のメタデータを持つポインターである、**ファットポインター**について話します。
名前が暗に意味する通り、それらはこの章で議論した**シンポインター**としても知られているポインターよりも大きいです。
