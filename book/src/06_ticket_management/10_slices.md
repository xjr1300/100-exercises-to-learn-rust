# Slices（スライス）

Let's go back to the memory layout of a `Vec`:

> `Vec`のメモリレイアウトに戻りましょう。

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

We already remarked how `String` is just a `Vec<u8>` in disguise.\
The similarity should prompt you to ask: "What's the equivalent of `&str` for `Vec`?"

> `String`が単なる`Vec<u8>`の変装であることをすでに述べました。
> この類似性は、「`Vec`に対して`&str`に相当するものはなんですか？」と質問することを促すはずです。

## `&[T]`

`[T]` is a **slice** of a contiguous sequence of elements of type `T`.\
It's most commonly used in its borrowed form, `&[T]`.

> `[T]`は、型`T`の連続した要素の**スライス**です。`
> それは、その借用された形式である`&[T]`として最も一般的に使用されます。

There are various ways to create a slice reference from a `Vec`:

> `Vec`からスライス参照を作成する方法がいくつかあります。

```rust
let numbers = vec![1, 2, 3];
// Via index syntax
// インデックス構文を介して
let slice: &[i32] = &numbers[..];
// Via a method
// メソッドを介して
let slice: &[i32] = numbers.as_slice();
// Or for a subset of the elements
// また、要素のサブセットの場合
let slice: &[i32] = &numbers[1..];
```

`Vec` implements the `Deref` trait using `[T]` as the target type, so you can use slice methods on a `Vec` directly
thanks to deref coercion:

> `Vec`は、目的の型として`[T]`を使用した`Deref`トレイトを実装しているため、参照外し型強制のおかげて、スライスメソッドに直接`Vec`を使用できます。

```rust
let numbers = vec![1, 2, 3];
// Surprise, surprise: `iter` is not a method on `Vec`!
// It's a method on `&[T]`, but you can call it on a `Vec`
// thanks to deref coercion.
// 驚き、驚き: `iter`は`Vec`のメソッドではありません！
// それは`&[T]`のメソッドですが、参照外し型強制のおかげで`Vec`でそれを呼び出せます。
let sum: i32 = numbers.iter().sum();
```

### Memory layout（メモリレイアウト）

A `&[T]` is a **fat pointer**, just like `&str`.\
It consists of a pointer to the first element of the slice and the length of the slice.

> `&[T]`は、ちょうど`&str`のように**ファットポインター**です。

If you have a `Vec` with three elements:

> 3つの要素を持つ`Vec`がある場合・・・

```rust
let numbers = vec![1, 2, 3];
```

and then create a slice reference:

> そして、スライスの参照を作成すると・・・

```rust
let slice: &[i32] = &numbers[1..];
```

you'll get this memory layout:

> 次のメモリレイアウトが得られます。

```text
                  numbers                          slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   3    |    4     |      |    |    |   2    |
      +----|----+--------+----------+      +----|----+--------+
           |                                    |
           |                                    |
           v                                    |
         +---+---+---+---+                      |
Heap:    | 1 | 2 | 3 | ? |                      |
         +---+---+---+---+                      |
               ^                                |
               |                                |
               +--------------------------------+
```

### `&Vec<T>` vs `&[T]`

When you need to pass an immutable reference to a `Vec` to a function, prefer `&[T]` over `&Vec<T>`.\
This allows the function to accept any kind of slice, not necessarily one backed by a `Vec`.

> 関数に`Vec`への不変参照を渡す必要があるとき、`&Vec<T>`よりも`&[T]`を選択してください。

For example, you can then pass a subset of the elements in a `Vec`.
But it goes further than that—you could also pass a **slice of an array**:

> 例えば、`Vec`の要素の部分集合を渡す事ができます。
> しかし、それはそれ以上の事です。**配列のスライス**も渡すことができます。

```rust
let array = [1, 2, 3];
let slice: &[i32] = &array;
```

Array slices and `Vec` slices are the same type: they're fat pointers to a contiguous sequence of elements.
In the case of arrays, the pointer points to the stack rather than the heap, but that doesn't matter
when it comes to using the slice.

> 配列のスライスと`Vec`のスライスは同じ型です。それらは連続した要素のシーケンスへのファットポインターです。
> 配列の場合、ポインターはヒープではなくスタックを指し示しますが、スライスを使用する際は問題ありません。
