# Leaking data（データのリーク）

The main concern around passing references to spawned threads is use-after-free bugs:
accessing data using a pointer to a memory region that's already been freed/de-allocated.\
If you're working with heap-allocated data, you can avoid the issue by
telling Rust that you'll never reclaim that memory: you choose to **leak memory**,
intentionally.

> 生み出したスレッドに参照を渡すことに関する主要な懸念は、すでに開放／割り当てを解除されたメモリ領域へのポインターを使用したデータアクセスという、「開放した後に使用する」バグです。
> ヒープに割り当てられたデータを扱っている場合、そのメモリを決して回収しないことをRustに伝えることで、その問題を回避できます。
> つまり、**メモリーリーク**することを意図的に選択します。

This can be done, for example, using the `Box::leak` method from Rust's standard library:

> 例えば、Rust標準ライブラリの`Box::leak`メソッドを使用して、これを行えます。

```rust
// Allocate a `u32` on the heap, by wrapping it in a `Box`.
// `Box`内にラップすることで、ヒープに`u32`を割り当てます。
let x = Box::new(41u32);
// Tell Rust that you'll never free that heap allocation
// using `Box::leak`. You can thus get back a 'static reference.
// `Box::leak`を使用して、そのヒープ割り当てを決して開放しないことをRustに伝えます。
// よって、`'static参照を得られます。
let static_ref: &'static mut u32 = Box::leak(x);
```

## Data leakage is process-scoped（データリークはプロセススコープです）

Leaking data is dangerous: if you keep leaking memory, you'll eventually
run out and crash with an out-of-memory error.

> データをリークさせることは危険です。
> リークしたメモリを維持する場合、最終的にメモリが尽きて、アウトオブメモリーエラーでクラッシュします。

```rust
// If you leave this running for a while,
// it'll eventually use all the available memory.
// しばらくこれを実行した場合、最終的にすべての利用可能なメモリを使用します。
fn oom_trigger() {
    loop {
        let v: Vec<usize> = Vec::with_capacity(1024);
        v.leak();
    }
}
```

At the same time, memory leaked via `leak` method is not truly forgotten.\
The operating system can map each memory region to the process responsible for it.
When the process exits, the operating system will reclaim that memory.

> 同時に、`leak`メソッドを介したメモリリークは、完全に忘れ去られるわけではありません。
> オペレーティングシステムは、それぞれのメモリ領域と、それに責任があるプロセスをマップできます。
> プロセスが終了したとき、オペレーティングシステムはそのメモリを回収します。

Keeping this in mind, it can be OK to leak memory when:

- The amount of memory you need to leak is not unbounded/known upfront, or
- Your process is short-lived and you're confident you won't exhaust
  all the available memory before it exits

> これを念頭に置いて、メモリリークが許容される場合は次のとおりです。
>
> - リークする必要があるメモリーの量が無制限でないか、事前にわかっている場合、または・・・
> - プロセスが短命で、それが終了する前に利用可能なメモリをすべて使い果たさない自信がある場合

"Let the OS deal with it" is a perfectly valid memory management strategy
if your usecase allows for it.

> ユースケースがそれ（上記）に従う場合、「OSにそれを処理させる」は、完全に妥当なメモリ管理戦略です。
