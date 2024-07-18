# Channels（チャネル）

All our spawned threads have been fairly short-lived so far.\
Get some input, run a computation, return the result, shut down.

> これまで生み出したスレッドは、かなり短命でした。
> 何らかの入力を受け取り、計算を実行して、結果を返して、終了します。

For our ticket management system, we want to do something different:
a client-server architecture.

> チケット管理システムでは、違ったことを行うつもりです。
> それは、クライアントサーバーアーキテクチャーです。

We will have **one long-running server thread**, responsible for managing
our state, the stored tickets.

> **1つの長時間実行するサーバースレッド**をもち、状態を管理する責任を持ち、チケットを保存します。

We will then have **multiple client threads**.\
Each client will be able to send **commands** and **queries** to
the stateful thread, in order to change its state (e.g. add a new ticket)
or retrieve information (e.g. get the status of a ticket).\
Client threads will run concurrently.

> そして、**複数のクライアントスレッド**を持ちます。
> それぞれのクライアントは、例えば新しいチケットを追加するように状態を変更する、または、例えばチケッtの状態を取得するように情報を得るために、状態を持ったスレッドに**コマンド**と**クエリ**を送信します。
> クライアントスレッドは、同時並行で実行します。

## Communication（コミュニケーション）

So far we've only had very limited parent-child communication:

- The spawned thread borrowed/consumed data from the parent context
- The spawned thread returned data to the parent when joined

> これまで、とても限定された親と子のコミュニケーションしかありませんでした。
>
> - 生み出したスレッドは、親文脈からのデータを借用／消費しました。
> - 生み出したスレッドは、結合されたとき、親にデータを返しました。

This isn't enough for a client-server design.\
Clients need to be able to send and receive data from the server thread
_after_ it has been launched.

> これは、クライアント・サーバーの設計で十分ではありません。
> クライアントは、それが起動した_後_で、サーバースレッドにデータを送信または受診できる必要があります。

We can solve the issue using **channels**.

> **チャネル**を使用して、その問題を解決できます。

## Channels（チャネル）

Rust's standard library provides **multi-producer, single-consumer** (mpsc) channels
in its `std::sync::mpsc` module.\
There are two channel flavours: bounded and unbounded. We'll stick to the unbounded
version for now, but we'll discuss the pros and cons later on.

> Rust標準ライブラリは**マルチプロデューサー、シングルコンシューマー**（mpsc）チャネルを、`std::sync::mpsc`モジュールで提供しています。
> 2つのチャネルの種類があります。それは拘束されたものと拘束されていないものです。
> 現時点で、後続されていないものに固執しますが、後で利点と欠点を議論する予定です。

Channel creation looks like this:

> チャネルの作成は次のようになります。

```rust
use std::sync::mpsc::channel;

let (sender, receiver) = channel();
```

You get a sender and a receiver.\
You call `send` on the sender to push data into the channel.\
You call `recv` on the receiver to pull data from the channel.

> 送信者と受信者を取得します。
> チャネルにデータをいれるために、送信者で`send`を呼び出します。
> チャネルから出たを引き出すために、受信者で`recv`を呼び出します。

### Multiple senders（複数の送信者）

`Sender` is clonable: we can create multiple senders (e.g. one for
each client thread) and they will all push data into the same channel.

> `Sender`はクローン可能です。クライアントスレッドそれぞれに1つの送信者など、複数の送信者を作成でき、それらは同じチャネルにデータを入れます。

`Receiver`, instead, is not clonable: there can only be a single receiver
for a given channel.

> 代わりに`Receiver`はクローンではありません。特定のチャネルに単独の受信者のみ存在できます。

That's what **mpsc** (multi-producer single-consumer) stands for!

> それが**mpsc**（マルチプロデューサーシングルコンシューマー）の意味です！

### Message type（メッセージの型）

Both `Sender` and `Receiver` are generic over a type parameter `T`.\
That's the type of the _messages_ that can travel on our channel.

> `Sender`と`Receiver`は、型パラメーター`T`に対してジェネリックです。
> それは、チャネルを旅行する_メッセージ_の型です。

It could be a `u64`, a struct, an enum, etc.

> それは、`u64`、構造体、列挙型などです。

### Errors（エラー）

Both `send` and `recv` can fail.\
`send` returns an error if the receiver has been dropped.\
`recv` returns an error if all senders have been dropped and the channel is empty.

`send`と`recv`両方とも失敗する可能性があります。

In other words, `send` and `recv` error when the channel is effectively closed.

> つまり、チャネルが実質的に閉じられたとき、`send`と`recv`はエラーは失敗します。
