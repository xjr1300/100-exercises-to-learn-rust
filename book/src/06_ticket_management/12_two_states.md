# Ticket ids（チケットのID）

Let's think again about our ticket management system.\
Our ticket model right now looks like this:

> チケット管理システムについて再び考えましょう。
> チケットモデルは次のようになっています。

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

One thing is missing here: an **identifier** to uniquely identify a ticket.\
That identifier should be unique for each ticket. That can be guaranteed by generating it automatically when
a new ticket is created.

> ここで1つの欠けています。チケットを一意に識別する**識別子**です。
> その識別子はそれぞれのチケットで一意になるべきです。
> それは、新しいチケットが作成されたとき、自動的に生成されることによって保証されます。

## Refining the model（モデルを洗練する）

Where should the id be stored?\
We could add a new field to the `Ticket` struct:

> IDをどこに保存するべきでしょうか？
> `Ticket`構造体に新しいフィールドを追加できます。

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

But we don't know the id before creating the ticket. So it can't be there from the get-go.\
It'd have to be optional:

> しかし、チケットを作成する前にIDはわかりません。よって、最初からそこに存在できません。
> それは、オプションでなければなりません。

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

That's also not ideal—we'd have to handle the `None` case every single time we retrieve a ticket from the store,
even though we know that the id should always be there once the ticket has been created.

> それもまた理想的ではありません。チケットをストアから取得するたびに、一旦、チケットが作成されたら、IDが常にそこにあることを知っているにも関わらず、`None`のケースを処理しなければなりません。

The best solution is to have two different ticket **states**, represented by two separate types:
a `TicketDraft` and a `Ticket`:

> 最善の解決策は、`TicketDraft`と`Ticket`の2つの異なる型によって表現される、2つのチケットの**状態**を持つことです。

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

A `TicketDraft` is a ticket that hasn't been created yet. It doesn't have an id, and it doesn't have a status.\
A `Ticket` is a ticket that has been created. It has an id and a status.\
Since each field in `TicketDraft` and `Ticket` embeds its own constraints, we don't have to duplicate logic
across the two types.

> `TicketDraft`は、まだ作成されていないチケットです。それはIDを持たず、また状態も持ちません。
> `Ticket`は、作成されたチケットです。それはIDと状態を持ちます。
> `TicketDraft`と`Ticket`のそれぞれのフィールドは、それ独自の制約を埋め込んでいるため、2つの型の間でロジックを重複しないようにしなければなりません。
