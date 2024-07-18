# Heap（ヒープ）

The stack is great, but it can't solve all our problems. What about data whose size is not known at compile time?
Collections, strings, and other dynamically-sized data cannot be (entirely) stack-allocated.
That's where the **heap** comes in.

> スタックは偉大ですが、スタックはすべてのプログラムを解決することができません。
> コンパイル時にデータのサイズがわからない場合、何が発生しますか？
> コレクション、文字列、そして他の動的なサイズを持つデータは、まったくスタックに割り当てられません。
> ここで、**ヒープ**が登場します。

## Heap allocations（ヒープ割り当て）

You can visualize the heap as a big chunk of memory—a huge array, if you will.\
Whenever you need to store data on the heap, you ask a special program, the **allocator**, to reserve for you
a subset of the heap. We call this interaction (and the memory you reserved) a **heap allocation**.
If the allocation succeeds, the allocator will give you a **pointer** to the start of the reserved block.

> 大きなメモリの塊、巨大な配列としてヒープを可視化できます。
> ヒープにデータを保存する必要があるときはいつでも、ヒープの部分集合を予約するために**アロケーター**という特別なプログラムに依頼します。
> 割り当てが成功した場合、アロケーターは予約されたブロックの開始を指す**ポインター**を与えます。

## No automatic de-allocation（自動開放なし）

The heap is structured quite differently from the stack.\
Heap allocations are not contiguous, they can be located anywhere inside the heap.

> ヒープはスタックとまったく異なる構造になっています。
> ヒープ割り当ては連続的ではなく、それらはヒープ内のどこでも配置されます。

```text
+---+---+---+---+---+---+-...-+-...-+---+---+---+---+---+---+---+
|  Allocation 1 | Free  | ... | ... |  Allocation N |    Free   |
+---+---+---+---+---+---+ ... + ... +---+---+---+---+---+---+---+
```

It's the allocator's job to keep track of which parts of the heap are in use and which are free.
The allocator won't automatically free the memory you allocated, though: you need to be deliberate about it,
calling the allocator again to **free** the memory you no longer need.

> ヒープのどの部分が使用中で、どこが空いているのかを追跡することは、アロケーターの仕事です。
> アロケーターは、割り当てられたメモリを自動的に開放しませんが、再度アロケーターを呼び出して、もはや必要のないメモリを**開放**することを、意図的にしなければなりません。

## Performance（性能）

The heap's flexibility comes at a cost: heap allocations are **slower** than stack allocations.
There's a lot more bookkeeping involved!\
If you read articles about performance optimization you'll often be advised to minimize heap allocations
and prefer stack-allocated data whenever possible.

> ヒープの柔軟性はコストが掛かります。ヒープ割り当てはスタック割り当てよりも**遅い**です。
> これらは、多くのブックキーピングが関係しています。
> パフォーマンス最適化の記事を読んだ場合、ときどきヒープ割り当てを最小にして、可能なときはいつでもスタック割り当てを選ぶように勧められます。

## `String`'s memory layout（Stringのメモリレイアウト）

When you create a local variable of type `String`,
Rust is forced to allocate on the heap[^empty]: it doesn't know in advance how much text you're going to put in it,
so it can't reserve the right amount of space on the stack.\
But a `String` is not _entirely_ heap-allocated, it also keeps some data on the stack. In particular:

- The **pointer** to the heap region you reserved.
- The **length** of the string, i.e. how many bytes are in the string.
- The **capacity** of the string, i.e. how many bytes have been reserved on the heap.

> `String`型のローカル変数を作成したとき、Rustはヒープの割り当てを強制されます。
> 変数にどれだけのテキストをいれるか事前に知ることはできないため、それはスタックに正しい量の領域を予約することができません。
> しかし、`String`は_完全に_ヒープ割り当てではなく、スタックにもいくつかデータを保持しています。特に・・・
>
> - 予約したヒープ領域への**ポインター**
> - 文字列の**長さ**、つまり文字列にあるバイト数
> - 文字列の**容量**、つまりヒープに予約されているバイト数

Let's look at an example to understand this better:

> これをよく理解するために、例を確認しましょう。

```rust
let mut s = String::with_capacity(5);
```

If you run this code, memory will be laid out like this:

> このコードを実行した場合、メモリは次のように配置されます。

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   0    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | ? | ? | ? | ? | ? |
       +---+---+---+---+---+
```

We asked for a `String` that can hold up to 5 bytes of text.\
`String::with_capacity` goes to the allocator and asks for 5 bytes of heap memory. The allocator returns
a pointer to the start of that memory block.\
The `String` is empty, though. On the stack, we keep track of this information by distinguishing between
the length and the capacity: this `String` can hold up to 5 bytes, but it currently holds 0 bytes of
actual text.

> 5バイトのテキストを保持できる`String`を要求しました。
> `String::with_capacity`はアロケーターに対して、ヒープメモリの５バイトを要求します。
> アロケーターは、そのメモリブロックの開始を指すポインターを返します。
> しかし、`String`は空です。
> スタックで長さと容量を区別することでこの情報を追跡します。
> この`String`は5バイトまで保持できますが、現在、実際のテキストの0バイトを保持します。

If you push some text into the `String`, the situation will change:

> `String`に任意のテキストを入れたとき、状況は変わります。

```rust
s.push_str("Hey");
```

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   3    |    5     |
      +--|  ----+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | y | ? | ? |
       +---+---+---+---+---+
```

`s` now holds 3 bytes of text. Its length is updated to 3, but capacity remains 5.
Three of the five bytes on the heap are used to store the characters `H`, `e`, and `y`.

> 現在、`s`は3バイトのテキストを保持しています。
> その長さは3に更新されますが、容量は5のままです。
> ヒープ上の5バイトの3バイトは、文字`H`、`e`、`y`を保存するために使用されます。

### `usize`

How much space do we need to store pointer, length and capacity on the stack?\
It depends on the **architecture** of the machine you're running on.

> スタックにポインター、長さそして容量を保存するために必要な領域はどれくらいでしょうか？
> それは、実行しているマシンの**アーキテクチャ**に依存します。

Every memory location on your machine has an [**address**](https://en.wikipedia.org/wiki/Memory_address), commonly
represented as an unsigned integer.
Depending on the maximum size of the address space (i.e. how much memory your machine can address),
this integer can have a different size. Most modern machines use either a 32-bit or a 64-bit address space.

> マシン上のそれぞれのメモリ位置は**アドレス**で、一般的に符号なし整数で表現されます。
> アドレス空間の最大サイズに依存して（つまり、マシンが処理できるメモリの量）、この整数はさまざまなサイズになります。
> 最も現代的なマシンは、32ビットまたは64ビットのアドレス空間のどちらかを使用します。

Rust abstracts away these architecture-specific details by providing the `usize` type:
an unsigned integer that's as big as the number of bytes needed to address memory on your machine.
On a 32-bit machine, `usize` is equivalent to `u32`. On a 64-bit machine, it matches `u64`.

> Rustは、`usize`型を提供することで、これらアーキテクチャ特有の詳細を抽象化します。
> `usize`は、マシン上のメモリを指し示すために必要なほど大きなバイト数をもつ符号なし整数です。
> 32ビットマシンで`usize`は`u32`と同等です。64ビットマシンでそれは`u64`と一致します。

Capacity, length and pointers are all represented as `usize`s in Rust[^equivalence].

> Rustにおいて、容量、長さそしてポインターは、すべての`usize`として表現されます。

### No `std::mem::size_of` for the heap（ヒープ用のstd::mem::size_ofはない）

`std::mem::size_of` returns the amount of space a type would take on the stack,
which is also known as the **size of the type**.

> `std::mem::size_of`は型がスタックに獲得する領域の量を返し、それは**型のサイズ**としても知られています。

> What about the memory buffer that `String` is managing on the heap? Isn't that
> part of the size of `String`?

> `String`がヒープ上で管理されるメモリバッファはどうでしょうか？
> それは、`String`のサイズの一部ではないですか？

No!\
That heap allocation is a **resource** that `String` is managing.
It's not considered to be part of the `String` type by the compiler.

> いいえ！
> ヒープ割り当ては、`String`が管理している**リソース**です。
> それは、コンパイラーによって`String`型の一部として考えられていません。

`std::mem::size_of` doesn't know (or care) about additional heap-allocated data
that a type might manage or refer to via pointers, as is the case with `String`,
therefore it doesn't track its size.

> `std::mem::size_of`は、ポインターを介して型が管理または参照するヒープに割り当てられた追加データを知らないし、気にしていません。
> よって、`std::mem::size_of`はそのサイズを追跡しません。

Unfortunately there is no equivalent of `std::mem::size_of` to measure the amount of
heap memory that a certain value is allocating at runtime. Some types might
provide methods to inspect their heap usage (e.g. `String`'s `capacity` method),
but there is no general-purpose "API" to retrieve runtime heap usage in Rust.\
You can, however, use a memory profiler tool (e.g. [DHAT](https://valgrind.org/docs/manual/dh-manual.html)
or [a custom allocator](https://docs.rs/dhat/latest/dhat/)) to inspect the heap usage of your program.

> 不運にも、ランタイムである値が割り当てられたヒープメモリの量を計測する`std::mem::size_of`と同等なものはありません。
> ある型は、それらのヒープ使用を調査するためのメソッド（例えば、`String`の`capacity`メソッド）を提供していますが、Rustではラインタイムでヒープ使用を取得する一般的な目的の「API」はありません。
> しかし、メモリプロファイラーツールを使用するか、プログラムのヒープ使用を調査する、例えば`DHAT`またはカスタムアロケーターを使用できます。

[^empty]: `std` doesn't allocate if you create an **empty** `String` (i.e. `String::new()`).
Heap memory will be reserved when you push data into it for the first time.
例えば`String::new()`などで**空**の`String`を作成した場合、`std`は割り当てしません。
最初にヒープにデータを入れたとき、ヒープメモリは予約されます。

[^equivalence]: The size of a pointer depends on the operating system too.
In certain environments, a pointer is **larger** than a memory address (e.g. [CHERI](https://blog.acolyer.org/2019/05/28/cheri-abi/)).
Rust makes the simplifying assumption that pointers are the same size as memory addresses,
which is true for most modern systems you're likely to encounter.
ポインターのサイズはオペレーティングシステムにも依存します。
特定の環境では、ポインターはメモリアドレスよりも**大きく**なります（例えば`CHERI`）。
Rustは、メモリアドレスと同じサイズであるという単純化された仮定をして、それは遭遇する可能性のある最も現代的なシステムで当てはまります。
