# Design review（設計レビュー）

Let's take a moment to review the journey we've been through.

> 少しの間、通過してきた旅をレビューしてみましょう。

## Lockless with channel serialization（チャネルのシリアライゼーションを使用したロックレス）

Our first implementation of a multithreaded ticket store used:

- a single long-lived thread (server), to hold the shared state
- multiple clients sending requests to it via channels from their own threads.

> マルチスレッドなチケットストアの最初の実装は、次を使用しました。
>
> - 共有された状態を保持する単一な中息するスレッド（サーバー）
> - 複数のクライアントが、それぞれ自身のスレッドからチャネルを介してリクエストを送信

No locking of the state was necessary, since the server was the only one modifying the state. That's because
the "inbox" channel naturally **serialized** incoming requests: the server would process them one by one.\
We've already discussed the limitations of this approach when it comes to patching behaviour, but we didn't
discuss the performance implications of the original design: the server could only process one request at a time,
including reads.

> サーバーは状態を変更する唯一のものであったため、状態のロックは必要ありませんでした。
> それは、「受診」チャネルが自然に入ってくるリクエストを**シリアライズ**したためです。サーバーはそれらを1つずつ処理しました。
> パッチの振る舞いに来たときに、この振る舞いの制限をすでに議論しましたが、オリジナルの設計の性能の影響については議論しませんでした。
> サーバーは、読み込みを含んで、一度に1つのリクエストしか処理できません。

## Fine-grained locking（適切な粒度のロック）

We then moved to a more sophisticated design, where each ticket was protected by its own lock and
clients could independently decide if they wanted to read or atomically modify a ticket, acquiring the appropriate lock.

> そして、より洗練された設計に移行して、それぞれのチケットがそれ自身のロックによって保護され、クライアントは独立で適切なロックを獲得して、チケットの読み込みかアトミックな修正かを決定できました。

This design allows for better parallelism (i.e. multiple clients can read tickets at the same time), but it is
still fundamentally **serial**: the server processes commands one by one. In particular, it hands out locks to clients
one by one.

> 例えば、複数クライアントが同時にチケットを読み込みできるように、この設計は良い並列処理を可能にしますが、それはまだ基本的に**シリアル**です。

Could we remove the channels entirely and allow clients to directly access the `TicketStore`, relying exclusively on
locks to synchronize access?

> 完全にチャネルを削除して、クライアントが直接`TicketStore`にアクセスして、同期的なアクセスをするために排他的なロックに依存することができるでしょうか？

## Removing channels（チャネルの削除）

We have two problems to solve:

- Sharing `TicketStore` across threads
- Synchronizing access to the store

> 解決するために問題が2つあります。
>
> - スレッドをまたいで`TicketStore`を共有する
> - ストアへの同期アクセス

### Sharing `TicketStore` across threads（スレッドをまたいだTicketStoreの共有）

We want all threads to refer to the same state, otherwise we don't really have a multithreaded system—we're just
running multiple single-threaded systems in parallel.\
We've already encountered this problem when we tried to share a lock across threads: we can use an `Arc`.

> すべてのスレッドが同じ状態を参照したいと考えていて、そうでないと、本物のマルチスレッドなシステムを持てません。
> 単に並列で複数の単一スレッドなシステムを実行しているだけです。
> スレッドをまたいでロックを共有することを試みたときに、この問題に遭遇しました。`Arc`を使用できます。

### Synchronizing access to the store（ストアへの同期アクセス）

There is one interaction that's still lockless thanks to the serialization provided by the channels: inserting
(or removing) a ticket from the store.\
If we remove the channels, we need to introduce (another) lock to synchronize access to the `TicketStore` itself.

> チャネルによって提供されたシリアライゼーションのおかげで、まだロックレスな相互作用が1つあります。
> それは、ストアにチケットを挿入（または削除）することです。
> チャネルを削除する場合、`TicketStore`自身に同期アクセスするために他のロックを導入する必要があります。

If we use a `Mutex`, then it makes no sense to use an additional `RwLock` for each ticket: the `Mutex` will
already serialize access to the entire store, so we wouldn't be able to read tickets in parallel anyway.\
If we use a `RwLock`, instead, we can read tickets in parallel. We just need to pause all reads while inserting
or removing a ticket.

> `Mutex`を使用する場合、それぞれのチケットに対して追加の`RwLock`を使用する意味がありません。
> `Mutex`は、すでにストア全体へのアクセスをシリアライズするため、並列でチケットを読み込むことができなくなります。
> 代わりに`RwLock`を使用した場合、並列でチケットを読み込むことができます。
> チケットの挿入または削除の間、単にすべての読み込みを停止する必要があります。

Let's go down this path and see where it leads us.

> この未知を進んで、どこに導かれるか確認しましょう。
