# Update operations（更新操作）

So far we've implemented only insertion and retrieval operations.\
Let's see how we can expand the system to provide an update operation.

> これまで、挿入と取得操作のみを実装してきました。
> 更新操作を提供するためにシステムを拡張する方法を確認しましょう。

## Legacy updates（従来の更新）

In the non-threaded version of the system, updates were fairly straightforward: `TicketStore` exposed a
`get_mut` method that allowed the caller to obtain a mutable reference to a ticket, and then modify it.

> システムの非スレッドバージョンにおいて、更新はかなり簡単です。
> `TicketStore`は、呼び出し側にチケットの可変参照を取得する`get_mut`メソッドを公開し、そしてそれを更新します。

## Multithreaded updates（マルチスレッドな更新）

The same strategy won't work in the current multi-threaded version,
because the mutable reference would have to be sent over a channel. The borrow checker would
stop us, because `&mut Ticket` doesn't satisfy the `'static` lifetime requirement of `SyncSender::send`.

> 同じ戦略は、可変参照はチャネルを通じて送信されなくてはならないため、現在のマルチスレッドバージョンでは機能しません。
> `&mut Ticket`は`SyncSender::send`の`'static`ライフタイム要求を満足しないため、借用チェッカーは止めます。

There are a few ways to work around this limitation. We'll explore a few of them in the following exercises.

> この制限を回避するいくつかの方法があります。
> 次の演習でそれらのいくつかを探求する予定です。

### Patching（パッチ）

We can't send a `&mut Ticket` over a channel, therefore we can't mutate on the client-side.\
Can we mutate on the server-side?

> チャネルで`&mut Ticket`を送信できないため、クライアント側で変更できません。
> サーバー側で変更できるでしょうか？

We can, if we tell the server what needs to be changed. In other words, if we send a **patch** to the server:

> サーバーに変更しなくてはならないものを伝えれば、できます。
> 言い換えれば、**パッチ**をサーバーに送信すればできます。

```rust
struct TicketPatch {
    id: TicketId,
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<TicketStatus>,
}
```

The `id` field is mandatory, since it's required to identify the ticket that needs to be updated.\
All other fields are optional:

- If a field is `None`, it means that the field should not be changed.
- If a field is `Some(value)`, it means that the field should be changed to `value`.

> `id`フィールドは必須なため、それは更新される必要があるチケットを識別するために要求されます。
> 他のフィールドすべてはオプションです。
>
> - フィールドが`None`の場合、それはそのフィールドが変更されるべきでないことを意味します。
> - フィールドが`Some(value)`の場合、それはそのフィールドが`value`に変更されるべきであることを意味します。
