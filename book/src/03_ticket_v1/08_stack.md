# Memory layout（メモリレイアウト）

We've looked at ownership and references from an operational point of view—what you can and can't do with them.
Now it's a good time to take a look under the hood: let's talk about **memory**.

> 所有権と参照を使用して何ができて、何ができないかを、操作の観点から確認しました。
> 現在、内部を確認する良いときです。

## Stack and heap（スタックとヒープ）

When discussing memory, you'll often hear people talk about the **stack** and the **heap**.\
These are two different memory regions used by programs to store data.

Let's start with the stack.

> メモリを議論するとき、時々**スタック**と**ヒープ**について話す人々をよく聞くでしょう。
> これらは、データを保存するためにプログラムから使用される2つの異なるメモリ領域です。

## Stack（スタック）

The **stack** is a **LIFO** (Last In, First Out) data structure.\
When you call a function, a new **stack frame** is added on top of the stack. That stack frame stores
the function's arguments, local variables and a few "bookkeeping" values.\
When the function returns, the stack frame is popped off the stack[^stack-overflow].

> **スタック**は**LIFO**（最後に入ったら、最初に出る）データ構造です。
> 関数を呼び出したとき、新しい**スタックフレーム**がスタックの最上部に追加されます。
> スタックフレームは関数の引数、ローカル変数、そして少しの「簿記」の値を保存します。
> 関数が戻ったとき、スタックフレームはスタックから取り出されます。

```text
                                 +-----------------+
                       func2     | frame for func2 |   func2
+-----------------+  is called   +-----------------+  returns   +-----------------+
| frame for func1 | -----------> | frame for func1 | ---------> | frame for func1 |
+-----------------+              +-----------------+            +-----------------+
```

From an operational point of view, stack allocation/de-allocation is **very fast**.\
We are always pushing and popping data from the top of the stack, so we don't need to search for free memory.
We also don't have to worry about fragmentation: the stack is a single contiguous block of memory.

> 操作の観点から、スタックの割り当て／開放は**とても早い**です。
> 常にスタックの最上部にデータを入れて、最上部からデータを取り出すため、空きメモリを探す必要はありません。
> また、フラグメントについて心配する必要もありません。
> スタックは、１つの連続したメモリブロックです。

### Rust

Rust will often allocate data on the stack.\
You have a `u32` input argument in a function? Those 32 bits will be on the stack.\
You define a local variable of type `i64`? Those 64 bits will be on the stack.\
It all works quite nicely because the size of those integers is known at compile time, therefore
the compiled program knows how much space it needs to reserve on the stack for them.

> Rustは、よくスタックにデータを割り当てます。
> 関数に`u32`の入力引数がありますか？その32ビットはスタックの上にあります。
> `i64`型のローカル変数を定義してますか？その64ビットはスタックの上にあります。
> それら整数のサイズは、コンパイル時にわかるため、そのすべてはとても素晴らしく機能します。
> よって、コンパイルされたプログラムは、それらのためにスタックに予約する必要がある領域の量を理解しています。

### `std::mem::size_of`

You can verify how much space a type would take on the stack
using the [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html) function.

For a `u8`, for example:

> 型がスタックにどれだけ多くの領域を取得するか、`std::mem::size_of`関数を使用することで検証できます。
>
> 例えば`u8`では・・・

```rust
// We'll explain this funny-looking syntax (`::<u8>`) later on.
// Ignore it for now.
// 後で、この面白く見える構文（`::<u8>`）を説明します。
// 現時点では無視してください。
assert_eq!(std::mem::size_of::<u8>(), 1);
```

1 makes sense, because a `u8` is 8 bits long, or 1 byte.

> `u8`は8ビット長または1バイトであるため、1は理にかなっています。

[^stack-overflow]: If you have nested function calls, each function pushes its data onto the stack when it's called but
it doesn't pop it off until the innermost function returns.
If you have too many nested function calls, you can run out of stack space—the stack is not infinite!
That's called a [**stack overflow**](https://en.wikipedia.org/wiki/Stack_overflow).
> ネストされた関数呼び出しがある場合、関数が呼び出されたとき、それぞれの関数はスタックに関数のデータをプッシュしますが、最も内側にある関数が戻るまで、そのデータは取り出されません。
> 非常に多くネストした関数呼び出しがある場合、スタック領域を使い果たすかもしれません。スタックは無限でありません。
> これは**スタック・オーバーフロー**と呼ばれます。
