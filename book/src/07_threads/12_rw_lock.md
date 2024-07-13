# Readers and writers（リーダーとライター）

Our new `TicketStore` works, but its read performance is not great: there can only be one client at a time
reading a specific ticket, because `Mutex<T>` doesn't distinguish between readers and writers.

> 新しい`TicketStore`は機能しますが、読み込み性能はあまり良くありません。
> `Mutex<T>`はリーダーとライターを区別しないため、特定のチケットを読み込むクライアントは一度に1クライアントしかいません。

We can solve the issue by using a different locking primitive: `RwLock<T>`.\
`RwLock<T>` stands for **read-write lock**. It allows **multiple readers** to access the data simultaneously,
but only one writer at a time.

> 異なるロック構成要素の`RwLock<T>`を使用することで、その問題を解決できます。
> `RwLock<T>`は、**読み書きロック**を表しています。
> それは、同時に**複数リーダー**をデータにアクセスさせますが、一度に1つのライターのみアクセスさせます。

`RwLock<T>` has two methods to acquire a lock: `read` and `write`.\
`read` returns a guard that allows you to read the data, while `write` returns a guard that allows you to modify it.

> `RwLock<T>`は、`read`と`write`の2つのロックを獲得するメソッドがあります。
> `read`はデータを読み取りさせるガードを返し、`write`はそれを修正させるガードを返します。

```rust
use std::sync::RwLock;

// An integer protected by a read-write lock
// 読み書きロックによって保護された整数です。
let lock = RwLock::new(0);

// Acquire a read lock on the RwLock
// RwLockの読み込み専用ロックを獲得します。
let guard1 = lock.read().unwrap();

// Acquire a **second** read lock
// while the first one is still active
// 最初の読み込み専用ロックが有効な間に、**2番目の**読み込み専用ロックを獲得します。
let guard2 = lock.read().unwrap();
```

## Trade-offs（二律背反）

On the surface, `RwLock<T>` seems like a no-brainer: it provides a superset of the functionality of `Mutex<T>`.
Why would you ever use `Mutex<T>` if you can use `RwLock<T>` instead?

> 表面上、`RwLock<T>`は考えるまでも無いように見えます。それは`Mutex<T>`の機能のスーパーセットを提供します。
> `RwLock<T>`を使用する代わりに、なぜまだ`Mutex<T>`を使用するのでしょうか。

There are two key reasons:

- Locking a `RwLock<T>` is more expensive than locking a `Mutex<T>`.\
  This is because `RwLock<T>` has to keep track of the number of active readers and writers, while `Mutex<T>`
  only has to keep track of whether the lock is held or not.
  This performance overhead is not an issue if there are more readers than writers, but if the workload
  is write-heavy `Mutex<T>` might be a better choice.
- `RwLock<T>` can cause **writer starvation**.\
  If there are always readers waiting to acquire the lock, writers might never get a chance to run.\
  `RwLock<T>` doesn't provide any guarantees about the order in which readers and writers are granted access to the lock.
  It depends on the policy implemented by the underlying OS, which might not be fair to writers.

> 主に2つの理由があります。
>
> - `RwLock<T>`をロックすることは、`Mutex<T>`をロックするよりもコストが高いです。
>   これは、`RwLock<T>`は有効なリーダーとライターの数を追跡する一方で、`Mutex<T>`はロックが保持されているかどうかを追跡するだけだからです。
>   この性能のオーバーヘッドは、ライターよりもリーダーのほうが多い場合は問題に成りませんが、ワークロードがライター重視の場合、`Mutex<T>`が良い選択かもしれません。
> - `RwLock<T>`は**ライターの飢餓**を率い起こす可能性があります。
>   ロックを獲得するために常にリーダーが待機している場合、ライターは実行する機会を得られないかもしれません。
>   `RwLock<T>`は、リーダとライターがロックにアクセスする権限を与えられる順序を保証しません。
>   それは、基盤となるOSによって実装された方針に依存して、ライターにとって公平でないかもしれません。

In our case, we can expect the workload to be read-heavy (since most clients will be reading tickets, not modifying them),
so `RwLock<T>` is a good choice.

> チケット管理システムの場合、ほとんどのクライアントはチケットを読み込み、チケットの修正はほとんどないため、ワークロードはリーダー重視であると期待できるため、`RwLock<T>`は良い選択です。
