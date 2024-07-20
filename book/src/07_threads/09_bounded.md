# Bounded vs unbounded channels（制限されたチャネルと非制限チャネル）

So far we've been using unbounded channels.\
You can send as many messages as you want, and the channel will grow to accommodate them.\
In a multi-producer single-consumer scenario, this can be problematic: if the producers
enqueues messages at a faster rate than the consumer can process them, the channel will
keep growing, potentially consuming all available memory.

> これまで、非制限チャネルを使用してきました。
> 望むだけ多くのメッセージを送信でき、チャネルはそれらを収容するために成長します。
> マルチプロデューサーシングルコンシューマーシナリオでは、これは問題になる可能性があります。
> プロデューサーが、コンシューマーがそれらを処理するよりも早くメッセージをキューに入れた場合、チャネルは成長し続け、利用可能なメモリをすべて消費する可能性があります。

Our recommendation is to **never** use an unbounded channel in a production system.\
You should always enforce an upper limit on the number of messages that can be enqueued using a
**bounded channel**.

> プロダクションシステムにおける推奨事項は、**決して**非制限チャネルを使用しないことです。
> **制限されたチャネル**を使用して、常にキューに入れられるメッセージの上限を強制するべきです。

## Bounded channels（制限されたチャネル）

A bounded channel has a fixed capacity.\
You can create one by calling `sync_channel` with a capacity greater than zero:

> 制限されたチャネルは固定された容量を持ちます。
> ゼロより大きい容量で`sync_channel`を呼び出すことで、それを作成できます。

```rust
use std::sync::mpsc::sync_channel;

let (sender, receiver) = sync_channel(10);
```

`receiver` has the same type as before, `Receiver<T>`.\
`sender`, instead, is an instance of `SyncSender<T>`.

> `receiver`は、前の`Receiver<T>`と同じ型を持ち、代わりに`sender`は`SyncSender<T>`のインスタンスです。

### Sending messages（メッセージの送信）

You have two different methods to send messages through a `SyncSender`:

- `send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will block and wait until there is space available.
- `try_send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will return `Err(TrySendError::Full(value))`, where `value` is the message that couldn't be sent.

> `SyncSender`を介してメッセージを送信する2つの異なるメソッドがあります。
>
> - `send`: チャネル内に空きがある場合、それはメッセージをキューに入れて`Ok(())`を返します。
>   チャネルがいっぱいの場合、それはブロックして、利用可能な空きができるまで待機します。
> - `try_send`: チャネル内に空きがある場合、それはメッセージをキューに入れて`Ok(())`を返します。
>   チャネルがいっぱいの場合、それは`Err(TrySendError::Full(value))`を返して、返される`value`は送信されなかったメッセージです。

Depending on your use case, you might want to use one or the other.

> ユースケースに合わせて、どちらかを使用できます。

### Backpressure（逆圧）

The main advantage of using bounded channels is that they provide a form of **backpressure**.\
They force the producers to slow down if the consumer can't keep up.
The backpressure can then propagate through the system, potentially affecting the whole architecture and
preventing end users from overwhelming the system with requests.

> 制限されたチャネルを使用する主な利点は、それらが**逆圧**の形式を提供することです。
> それら（逆圧）は、コンシューマー（サーバー）がついていけなくなった場合、遅くすることをプロデューサー（クライアント）に強制します。
> そして、逆圧はシステムを介して伝播でき、全体のアーキテクチャーに影響を与える可能性があり、エンドユーザーがリクエストでシステムを圧倒することを回避します。
