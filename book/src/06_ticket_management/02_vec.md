# Vectors（ベクター）

Arrays' strength is also their weakness: their size must be known upfront, at compile-time.
If you try to create an array with a size that's only known at runtime, you'll get a compilation error:

> 配列の強みは、それらの弱点でもあります。それらのサイズは、コンパイル時に前もってわかっていなくてはなりません。
> ランタイム時しかわからないサイズを持つ配列を作成することを試みた場合、コンパイルエラーを得ます。

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

Arrays wouldn't work for our ticket management system—we don't know how many tickets we'll need to store at compile-time.
This is where `Vec` comes in.

> 配列は、チケット管理システムには適していません。
> コンパイル時に保存する必要のあるチケットの数を知ることができません。
> ここで`Vec`が登場します。

## `Vec`

`Vec` is a growable array type, provided by the standard library.\
You can create an empty array using the `Vec::new` function:

> `Vec`は成長する配列型で、標準ライブラリで提供されています。
> `Vec::new`関数を使用して空の配列を作成できます。

```rust
let mut numbers: Vec<u32> = Vec::new();
```

You would then push elements into the vector using the `push` method:

> その後、`push`メソッドを使用してベクター内に要素を追加します。

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

New values are added to the end of the vector.\
You can also create an initialized vector using the `vec!` macro, if you know the values at creation time:

> 新しい値はベクターの末尾に追加されます。
> 作成時に値がわかっている場合、`vec!`マクロを使用して初期化されたベクターを作成することもできます。

```rust
let numbers = vec![1, 2, 3];
```

## Accessing elements（要素へのアクセス）

The syntax for accessing elements is the same as with arrays:

> 要素にアクセスする構文は、配列と同じです。

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
You can also use the `get` method, which returns an `Option<&T>`:

> インデックスは`usize`型でなくてはなりません。
> `Option<&T>`を返す`get`メソッドを使用することもできます。

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

Access is bounds-checked, just like element access with arrays. It has O(1) complexity.

> アクセスは境界チェックされ、ちょうど配列の要素アクセスと同じです。
> それは、O(1)の複雑さがあります。

## Memory layout（メモリレイアウト）

`Vec` is a heap-allocated data structure.\
When you create a `Vec`, it allocates memory on the heap to store the elements.

> `Vec`はヒープに割り当てられたデータ構造です。
> `Vec`を作成するとき、それは要素を保存するためにヒープにメモリを割り当てます。

If you run the following code:

> 次のコードを実行した場合・・・

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

you'll get the following memory layout:

> 次のメモリレイアウトを得ます。

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

`Vec` keeps track of three things:

- The **pointer** to the heap region you reserved.
- The **length** of the vector, i.e. how many elements are in the vector.
- The **capacity** of the vector, i.e. the number of elements that can fit in the space reserved on the heap.

> `Vec`は次の3つを追跡します。
>
> - 予約したヒープ領域への**ポインター**
> - 例えばベクター内にある要素の数である、ベクターの**長さ**
> - 例えばヒープに予約された領域に収められる要素の数である、ベクターの**容量**

This layout should look familiar: it's exactly the same as `String`!\
That's not a coincidence: `String` is defined as a vector of bytes, `Vec<u8>`, under the hood:

> このレイアウトは慣れているはずです。それは`String`と正確に同じです。
> これは偶然の一致ではありません。`String`は、内部的にバイトのベクターである`Vec<u8>`として定義されています。

```rust
pub struct String {
    vec: Vec<u8>,
}
```
