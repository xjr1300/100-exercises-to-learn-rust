# `Sync`

Before we wrap up this chapter, let's talk about another key trait in Rust's standard library: `Sync`.

> このチャプターを仕上げる前に、Rust標準ライブラリにある他の主要なトレイトである`Sync`について話しましょう。

`Sync` is an auto trait, just like `Send`.\
It is automatically implemented by all types that can be safely **shared** between threads.

> `Sync`は、ちょうど`Send`と同様に自動トレイトです。
> それは、スレッド間で安全に**共有**できるすべての型に自動的に実装されます。

In order words: `T: Sync` means that `&T` is `Send`.

> 言い換えれば、`T: Sync`は、`&T`が`Send`であることを意味します。

## `Sync` doesn't imply `Send`（SyncはSendを暗に意味しない）

It's important to note that `Sync` doesn't imply `Send`.\
For example: `MutexGuard` is not `Send`, but it is `Sync`.

> `Sync`が`Send`を暗に意味しないことは重要な注意事項です。
> 例えば、`MutexGuard`は`Send`ではありませんが、`Sync`です。

> `&MutexGuard`は`Send`であるが、`MutexGuard`は`Send`でない。

It isn't `Send` because the lock must be released on the same thread that acquired it, therefore we don't
want `MutexGuard` to be dropped on a different thread.\
But it is `Sync`, because giving a `&MutexGuard` to another thread has no impact on where the lock is released.

> ロックは、それを獲得した同じスレッドで開放されなければならないため、`MutexGuard`が異なるスレッドでドロップされることを望んでないため、`MutexGuard`は`Send`ではありません。
> しかし、`MutexGuard`は`Sync`で、それは`&MutexGuard`を他のスレッドに与えることは、ロックが開放される場所に影響を与えないからです。

> `MutexGuard`のロックを開放するためには、所有権または可変参照が必要になる。
> よって、`&MutexGuard`では、ロックを開放できない。

## `Send` doesn't imply `Sync`（SendはSyncを暗に意味しない）

The opposite is also true: `Send` doesn't imply `Sync`.\
For example: `RefCell<T>` is `Send` (if `T` is `Send`), but it is not `Sync`.

> また反対も成立します。`Send`は`Sync`を暗に意味しません。

`RefCell<T>` performs runtime borrow checking, but the counters it uses to track borrows are not thread-safe.
Therefore, having multiple threads holding a `&RefCell` would lead to a data race, with potentially
multiple threads obtaining mutable references to the same data. Hence `RefCell` is not `Sync`.\
`Send` is fine, instead, because when we send a `RefCell` to another thread we're not
leaving behind any references to the data it contains, hence no risk of concurrent mutable access.

> `RefCell<T>`は、ラインタイムで借用チェックを行いますが、借用を追跡するためにそれが使用するカウンターはスレッドセーフではありません。
> よって、`&RefCell`を保持する複数のスレッドを持つことは、データ競合を引き起こし、潜在的に複数のスレッドが同じデータへの可変参照を取得する可能性があります。
> よって、`RefCell`は`Sync`ではありません。
> 代わりに`Send`は問題ありません。なぜなら、他のスレッドに`RefCell`を送信したとき、それが含むデータへの参照を背後に残すことはないためで、並行な可変アクセスのリスクはありません。
