# String slices（文字列スライス）

Throughout the previous chapters you've seen quite a few **string literals** being used in the code,
like `"To-Do"` or `"A ticket description"`.
They were always followed by a call to `.to_string()` or `.into()`. It's time to understand why!

> 前の章を通じて、コード内で使用された`"To-Do"`または`"A ticket description"`のような、**文字列リテラル**をいくつか見てきました。
> それらは常に、`to_string()`または`into()`の呼び出しで続きました。それがなぜなのかを理解する時がきました！

## String literals（文字列リテラル）

You define a string literal by enclosing the raw text in double quotes:

> ダブルウォート内に素のテキストを囲むことで、文字列リテラルを定義します。

```rust
let s = "Hello, world!";
```

The type of `s` is `&str`, a **reference to a string slice**.

> `s`の型は`&str`で、**文字列スライスへの参照**です。

## Memory layout（メモリレイアウト）

`&str` and `String` are different types—they're not interchangeable.\
Let's recall the memory layout of a `String` from our
[previous exploration](../03_ticket_v1/09_heap.md).
If we run:

> `&str`と`String`は異なる型で、それらは交換できません。
> 前の探求から`String`のメモリレイアウトを思い出しましょう。
> もし、次を実行した場合・・・

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
```

we'll get this scenario in memory:

> メモリ上で次のシナリオが得られます。

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

If you remember, we've [also examined](../03_ticket_v1/10_references_in_memory.md)
how a `&String` is laid out in memory:

> 覚えている場合、メモリ内で`&String`が配置される方法も調査しました。

```text
     --------------------------------------
     |                                    |
+----v----+--------+----------+      +----|----+
| pointer | length | capacity |      | pointer |
|    |    |   5    |    5     |      |         |
+----|----+--------+----------+      +---------+
     |        s                          &s
     |
     v
   +---+---+---+---+---+
   | H | e | l | l | o |
   +---+---+---+---+---+
```

`&String` points to the memory location where the `String`'s metadata is stored.\
If we follow the pointer, we get to the heap-allocated data. In particular, we get to the first byte of the string, `H`.

> `&String`は、`String`のメタデータを保存したメモリの場所を指し示しています。
> ポインタをたどった場合、ヒープに割り当てられたデータに到達します。特に、文字列の最初のバイトの`H`に到達します。

What if we wanted a type that represents a **substring** of `s`? E.g. `ello` in `Hello`?

> 例えば、`Hello`内の`ello`のように、`s`の**部分文字列**を表現する型が欲しい場合どうなるでしょうか？

## String slices（文字列スライス）

A `&str` is a **view** into a string, a **reference** to a sequence of UTF-8 bytes stored elsewhere.
You can, for example, create a `&str` from a `String` like this:

> `&str`は文字列内部への**ビュー**であり、他の場所に保存されたUTF-8バイトシーケンスへの参照です。
> 例えば、次のように`String`から`&str`を作成できます。

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, skipping the first byte.
// 最初のバイトをスキップして、`String`から文字列スライスの参照を作成します。
let slice: &str = &s[1..];
```

In memory, it'd look like this:

> メモリ内で、それは次のように見えます。

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

`slice` stores two pieces of information on the stack:

- A pointer to the first byte of the slice.
- The length of the slice.

> `スライス`は、スタック上の2つの情報を蓄積します。
>
> - スライスの最初のバイトへのポインター
> - スライスの長さ

`slice` doesn't own the data, it just points to it: it's a **reference** to the `String`'s heap-allocated data.\
When `slice` is dropped, the heap-allocated data won't be deallocated, because it's still owned by `s`.
That's why `slice` doesn't have a `capacity` field: it doesn't own the data, so it doesn't need to know how much
space it was allocated for it; it only cares about the data it references.

> `スライス`は単にデータを指し示すため、`スライス`はデータを所有しません。それは`String`のヒープに割り当てられたデータへの**参照**です。
> `スライス`がドロップされたとき、それはまだ`s`によって所有されているため、ヒープに割り当てられたデータは開放されません。
> それが、`スライス`が`capacity`フィールドを持たない理由です。それはデータを所有しないため、それはそのために割り当てられた領域の量を知る必要はありません。
> 単にそれは、それが参照するデータについてのみ気にかけています。

## `&str` vs `&String`（&strと&String）

As a rule of thumb, use `&str` rather than `&String` whenever you need a reference to textual data.\
`&str` is more flexible and generally considered more idiomatic in Rust code.

> 経験則から、テキストデータへの参照が必要なときはいつでも、`&String`よりも`&str`を使用してください。

If a method returns a `&String`, you're promising that there is heap-allocated UTF-8 text somewhere that
**matches exactly** the one you're returning a reference to.\
If a method returns a `&str`, instead, you have a lot more freedom: you're just saying that _somewhere_ there's a
bunch of text data and that a subset of it matches what you need, therefore you're returning a reference to it.

> メソッドが`&String`を返す場合、参照を返すモノと**完全に一致する**、ヒープに割り当てられたUTF-8テキストがどこかにあることを約束しています。
> 代わりにメソッドが`&str`を返す場合、より自由度が高くなります。
> どこかにテキストデータの束があり、そのサブセットが必要なものと一致することを単純に述べているため、それへの参照を返しています。
