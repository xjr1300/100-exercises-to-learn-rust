# `Sized`

There's more to `&str` than meets the eye, even after having
investigated deref coercion.\
From our previous [discussion on memory layouts](../03_ticket_v1/10_references_in_memory.md),
it would have been reasonable to expect `&str` to be represented as a single `usize` on
the stack, a pointer. That's not the case though. `&str` stores some **metadata** next
to the pointer: the length of the slice it points to. Going back to the example from
[a previous section](06_str_slice.md):

> 参照外し型強制を調査した後でも、目に見える以上のことが`&str`にあります。
> 前のメモリレイアウトで議論した内容から、それは`&str`がスタック上の単一の`usize`、つまりポインターとして表現されることを期待することは理由があります。
> しかし、この場合は違い、`&str`はポインターの隣にいくつかの**メタデータ**を保存しています。
> それが指し示すスライスの長さです。
> 前の節の例に戻ります。

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, skipping the first byte.
// 最初のバイトをスキップした、`String`からの文字列スライスの参照を作成します。
let slice: &str = &s[1..];
```

In memory, we get:

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |
           |                                    |
           v                                    |
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

What's going on?

> 何が起こっているのでしょうか？

## Dynamically sized types（動的サイズ型）

`str` is a **dynamically sized type** (DST).\
A DST is a type whose size is not known at compile time. Whenever you have a
reference to a DST, like `&str`, it has to include additional
information about the data it points to. It is a **fat pointer**.\
In the case of `&str`, it stores the length of the slice it points to.
We'll see more examples of DSTs in the rest of the course.

> `str`は**動的サイズ型**（DST）です。
> DSTは、コンパイル時にサイズがわからない型です。
> `&str`のような、DSTへの参照を持つときはいつでも、それはそれが指し示すデータについての追加情報を含む必要があります。
> それは**ファットポインター**です。
> `&str`の場合、それはそれが指し示すスライスの長さを保存します。
> コースの残りで、DSTのさらなる例を確認します。

## The `Sized` trait（`Sized`トレイト）

Rust's `std` library defines a trait called `Sized`.

> Rustの`std`ライブラリは、`Sized`と呼ばれるトレイトを定義しています。

```rust
pub trait Sized {
    // This is an empty trait, no methods to implement.
    // これは空のトレイトで、実装するメソッドはありません。
}
```

A type is `Sized` if its size is known at compile time. In other words, it's not a DST.

> コンパイル時にそのサイズが分かっている場合、その型は`Sized`です。言い換えれば、DSTではありません。

### Marker traits（マーカートレイト）

`Sized` is your first example of a **marker trait**.\
A marker trait is a trait that doesn't require any methods to be implemented. It doesn't define any behavior.
It only serves to **mark** a type as having certain properties.
The mark is then leveraged by the compiler to enable certain behaviors or optimizations.

> `Sized`は**マーカートレイト**の最初の例です。
> マーカートレイトは、実装されるメソッドを要求しないトレイトです。それは何の振る舞いも定義しません。
> それは、特定の属性を持つ型として**マーク**することのみを提供します。
> そして、マークは特定の振る舞いまたは最適化を有効にするためにコンパイラーによって利用されます。

### Auto traits（自動トレイト）

In particular, `Sized` is also an **auto trait**.\
You don't need to implement it explicitly; the compiler implements it automatically for you
based on the type's definition.

> 特に、`Sized`は**自動トレイト**でもあります。
> それを明示的に実装する必要はありません。
> コンパイラーは、それを型の定義に基づいて自動的に実装します。

### Examples（例）

All the types we've seen so far are `Sized`: `u32`, `String`, `bool`, etc.

> これまでに見てきた`u32`、`String`、`bool`などのすべての型は`Sized`です。

`str`, as we just saw, is not `Sized`.\
`&str` is `Sized` though! We know its size at compile time: two `usize`s, one for the pointer
and one for the length.

> ちょうど確認した通り、`str`は`Sized`ではありません。
> しかし、`&str`は`Sized`です！
> コンパイル時にそのサイズを知っています。
> 2つの`usize`で、そのポインターとその長さです。
