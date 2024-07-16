# Locks, `Send` and `Arc`（ロック、SendそしてArc）

The patching strategy you just implemented has a major drawback: it's racy.\
If two clients send patches for the same ticket roughly at same time, the server will apply them in an arbitrary order.
Whoever enqueues their patch last will overwrite the changes made by the other client.

> ちょうど実装したパッチ戦略は大きな決定があります。それは際どいです。
> 2つのクライアントがほぼ同時に同じチケットにパッチを送信した場合、サーバーは任意の順序でそれらを適用します。
> 最後にそれらのパッチをキューに入れた人が、他のクライアントによって行われた変更を上書きします。

## Version numbers（バージョン番号）

We could try to fix this by using a **version number**.\
Each ticket gets assigned a version number upon creation, set to `0`.\
Whenever a client sends a patch, they must include the current version number of the ticket alongside the
desired changes. The server will only apply the patch if the version number matches the one it has stored.

> **バージョン番号**を使用して、これを修正することを試みます。
> それぞれのチケットは、作成時に`0`に設定されたバージョン番号を割り当てられます。
> クライアントがパッチを送信したときはいつでも、それら（パッチ）は希望する変更と一緒に、チケットの現在のバージョン番号を含まなくてはなりません。
> サーバーは、バージョン番号がそれ（サーバー）が保存したものと一致した場合にのみパッチを適用します。

In the scenario described above, the server would reject the second patch, because the version number would
have been incremented by the first patch and thus wouldn't match the one sent by the second client.

> 上記で説明したシナリオにおいて、バージョン番号は最初のパッチによってい増加されており、2番目のクライアントによって送信されたバージョン番号と一致しないため、サーバーは2番目のパッチを拒否します。

This approach is fairly common in distributed systems (e.g. when client and servers don't share memory),
and it is known as **optimistic concurrency control**.\
The idea is that most of the time, conflicts won't happen, so we can optimize for the common case.
You know enough about Rust by now to implement this strategy on your own as a bonus exercise, if you want to.

> この方法は、例えばクライアントとサーバーがメモリを共有していないなどの分散システムでとても一般的で、それは**楽観的同時実行制御**として知られています。
> その愛では、ほとんどの時間で衝突が発生しないため、一般的なケースに最適化できます。
> そろそれRustを十分に理解したため、希望する場合、ボーナスの演習として、自身でこの戦略を実装してください。

## Locking（ロック）

We can also fix the race condition by introducing a **lock**.\
Whenever a client wants to update a ticket, they must first acquire a lock on it. While the lock is active,
no other client can modify the ticket.

> また、**ロック**を導入することで、競合状態を修正することもできます。
> クライアントがチケットを更新したいときはいつでも、それら（クライアント）は、最初にそれ（チケット）のロックを取得しなければなりません。
> ロックが有効な間、他のクライアントはチケットを修正できません。

Rust's standard library provides two different locking primitives: `Mutex<T>` and `RwLock<T>`.\
Let's start with `Mutex<T>`. It stands for **mut**ual **ex**clusion, and it's the simplest kind of lock:
it allows only one thread to access the data, no matter if it's for reading or writing.

> Rust標準ライブラリは、`Mutex<T>`と`RwLock<T>`の2つの異なるロックの構成要素を提供しています。
> `Mutex<T>`から始めましょう。それは**可変**で、**排他**を表していて、それは最も単純な種類のロックです。
> それは、読み込みまたは書き込みに関わらず、データへのアクセスをたった1つのスレッドだけ許可します。

`Mutex<T>` wraps the data it protects, and it's therefore generic over the type of the data.\
You can't access the data directly: the type system forces you to acquire a lock first using either `Mutex::lock` or
`Mutex::try_lock`. The former blocks until the lock is acquired, the latter returns immediately with an error if the lock
can't be acquired.\
Both methods return a guard object that dereferences to the data, allowing you to modify it. The lock is released when
the guard is dropped.

> `Mutex<T>`は、それが保護するデータをラップして、そのためそれはデータの型に対してジェネリックです。
> 直接データにアクセスできません。型システムは、`Mutex::lock`または`Mutex::try_lock`のどちらかを使用して、最初にロックを獲得することを強制します。
> 前者はロックが獲得されるまでブロックして、後者はロックが獲得できない場合、即座にエラーを返します。
> 両方のメソッドは、データへの参照を外すガードオブジェクトを返し、それ（データ）を変更できるようにします。
> ロックは、ガードがドロップされたときに、開放されます。

```rust
use std::sync::Mutex;

// An integer protected by a mutex lock
// ミューテックスロックによって保護された整数です。
let lock = Mutex::new(0);

// Acquire a lock on the mutex
// ミューテックスのロックを獲得します。
let mut guard = lock.lock().unwrap();

// Modify the data through the guard,
// leveraging its `Deref` implementation
// ガードの`Deref`実装を利用して、ガードを介してデータを修正します。
*guard += 1;

// The lock is released when `data` goes out of scope
// This can be done explicitly by dropping the guard
// or happen implicitly when the guard goes out of scope
// `data`がスコープ外になったとき、ロックは開放されます。
// これは、ガードをドロップすることによって明示に行うか、ガードがスコープ外になったときに暗黙的に発生します。
drop(guard)
```

## Locking granularity（ロックの粒度）

What should our `Mutex` wrap?\
The simplest option would be the wrap the entire `TicketStore` in a single `Mutex`.\
This would work, but it would severely limit the system's performance: you wouldn't be able to read tickets in parallel,
because every read would have to wait for the lock to be released.\
This is known as **coarse-grained locking**.

> 何を`Mutex`でラップするべきでしょうか？
> 単純な選択肢は、`TicketStore`全体を単一の`Mutex`でラップすることです。
> これは機能しますが、システムの性能を著しく制限します。
> すべての読み込みはロックの解放を待たなければならないため、並列してチケットを読むことができなくなります。
> これは、**粗い粒度のロック**として知られています。

It would be better to use **fine-grained locking**, where each ticket is protected by its own lock.
This way, clients can keep working with tickets in parallel, as long as they aren't trying to access the same ticket.

> それぞれのチケットがそれ自身をロックすることによって保護する、**適切な粒度のロック**を使用したほうが良いです。
> この方法では、クライアントは、同じチケットにアクセスすることを試みない限り、並列でチケットを操作し続けることができます。

```rust
// The new structure, with a lock for each ticket
// それぞれのチケットのロックを持つ新しい構造体です。
struct TicketStore {
    tickets: BTreeMap<TicketId, Mutex<Ticket>>,
}
```

This approach is more efficient, but it has a downside: `TicketStore` has to become **aware** of the multithreaded
nature of the system; up until now, `TicketStore` has been blissfully ignoring the existence of threads.\
Let's go for it anyway.

> この方法はより効率的ですが、欠点があります。
> `TicketStore`は、システムのマルチスレッド性に**気付く**ようにならなくてはなりません。
> これまで、`TicketStore`はスレッドの存在を穏やかに無視してきました。
> とにかくやってみましょう。

## Who holds the lock?（誰がロックを保持するのか？）

For the whole scheme to work, the lock must be passed to the client that wants to modify the ticket.\
The client can then directly modify the ticket (as if they had a `&mut Ticket`) and release the lock when they're done.

> 全体のスキームが機能するために、ロックはそのチケットを修正したいクライアントに渡されなければなりません。
> そして、クライアントは、`&mut Ticket`を持っている場合、チケットを直接修正して、終了したときにロックを開放できます。

This is a bit tricky.\
We can't send a `Mutex<Ticket>` over a channel, because `Mutex` is not `Clone` and
we can't move it out of the `TicketStore`. Could we send the `MutexGuard` instead?

> これは少しトリッキーです。
> `Mutex`は`Clone`でなく、`TicketStore`の外に移動できないため、チャネルを介して`Mutex<Ticket>`を送信することができません。
> 代わりに`MutexGuard`を送信できるでしょうか？

Let's test the idea with a small example:

> 小さな例でそのアイデアをテストしましょう。

```rust
use std::thread::spawn;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;

fn main() {
    let lock = Mutex::new(0);
    let (sender, receiver) = sync_channel(1);
    let guard = lock.lock().unwrap();

    spawn(move || {
        receiver.recv().unwrap();
    });

    // Try to send the guard over the channel
    // to another thread
    // チャネルを介して、他のスレッドにガードを送信することを試みます。
    sender.send(guard);
}
```

The compiler is not happy with this code:

> コンパイラーはこのコードに満足していません。

```text
error[E0277]: `MutexGuard<'_, i32>` cannot be sent between threads safely
   --> src/main.rs:10:7
    |
10  |   spawn(move || {
    |  _-----_^
    | | |
    | | required by a bound introduced by this call
11  | |     receiver.recv().unwrap();
12  | | });
    | |_^ `MutexGuard<'_, i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `MutexGuard<'_, i32>`, which is required by `{closure@src/main.rs:10:7: 10:14}: Send`
    = note: required for `std::sync::mpsc::Receiver<MutexGuard<'_, i32>>` to implement `Send`
note: required because it's used within this closure
```

`MutexGuard<'_, i32>` is not `Send`: what does it mean?

> `MutexGuard<'_, i32>`は`Send`ではありません。それは何を意味するのでしょうか？

## `Send`

`Send` is a marker trait that indicates that a type can be safely transferred from one thread to another.\
`Send` is also an auto-trait, just like `Sized`; it's automatically implemented (or not implemented) for your type
by the compiler, based on its definition.\
You can also implement `Send` manually for your types, but it requires `unsafe` since you have to guarantee that the
type is indeed safe to send between threads for reasons that the compiler can't automatically verify.

> `Send`は、あるスレッドから他のスレッドに安全に転送できる型を示すマーカートレイトです。
> また`Send`は、ちょうど`Sized`と同様に自動トレイトです。
> それ（`Send`）は、その（型）の定義に基づいて、コンパイラーによって自動的に実装されます（または実装されません）。
> また、型に手動で`Send`を実装することもできますが、コンパイラーは自動て検証できない理由で、型が確実にスレッド間を安全に送信できることを保証しなくてはならないため、`unsafe`が要求されます。

> `Send`を実装した型は、スレッド間で安全に移動（ムーブ）できる。
> また、`Sync`を実装した型は、複数のスレッドから安全に参照できる。
>
> 例えば、`&T`が`Send`である場合、`T`は`Sync`である。
>
> 型`T`が`Sync`である場合、複数のスレッドから安全に参照できるということは、`&T`が複数のスレッドで安全であることを意味する。
> そして、`&T`が`Send`である場合、`&T`はスレッド間を移動（ムーブ）できる。
> `&T`はスレッド間を移動できるということは、`T`が複数のスレッドで安全であり、`T`が`Sync`であることを意味する。

### Channel requirements（チャネルの要求事項）

`Sender<T>`, `SyncSender<T>` and `Receiver<T>` are `Send` if and only if `T` is `Send`.\
That's because they are used to send values between threads, and if the value itself is not `Send`, it would be
unsafe to send it between threads.

> `Sender<T>`、`SyncSender<T>`そして`Receive<T>`は`Send`で、かつ`T`が`Send`である場合のみ`Send`です。
> それは、それらがスレッド間で値を送信するために使用されることが理由で、その値自身が`Send`でない場合、スレッド間でそれを安全に送信できないからです。

### `MutexGuard`

`MutexGuard` is not `Send` because the underlying operating system primitives that `Mutex` uses to implement
the lock require (on some platforms) that the lock must be released by the same thread that acquired it.\
If we were to send a `MutexGuard` to another thread, the lock would be released by a different thread, which would
lead to undefined behavior.

> `MutexGuard`は、
> `Mutex`がロックを実装するために使用する基盤となるオペレーティングシステムの構成要素は、（いくつかのプラットフォームで）ロックは、それを獲得した同じスレッドによって開放されなくてはならないことを要求します。
> 他のスレッドに`MutexGuard`を送信した場合、ロックは異なるスレッドによって開放される音になり、それは未定義な動作を引き起こします。

## Our challenges（課題）

Summing it up:

- We can't send a `MutexGuard` over a channel. So we can't lock on the server-side and then modify the ticket on the
  client-side.
- We can send a `Mutex` over a channel because it's `Send` as long as the data it protects is `Send`, which is the
  case for `Ticket`.
  At the same time, we can't move the `Mutex` out of the `TicketStore` nor clone it.

> まとめると・・・
>
> - チャネルを介して`MutexGuard`を送信できません。よって、サーバー側でロックできず、クライアント側でチケットを修正できません。
> - `Mutex`は、それが保護するデータが`Send`である限り`Send`で、それは`Ticket`の場合であり、チャネルを介して`Mutex`を送信できます。
>   同時に、`TicketStore`の外側に`Mutex`を移動できず、それをクローンすることもできません。

How can we solve this conundrum?\
We need to look at the problem from a different angle.
To lock a `Mutex`, we don't need an owned value. A shared reference is enough, since `Mutex` uses internal mutability:

> どのようにこの難問を解決できるでしょうか？
> 異なる角度から問題を眺める必要があります。
> `Mutex`をロックするために、値を所有する必要はありません。`Mutex`は内部可変性を使用するため、共有参照で十分です。

```rust
impl<T> Mutex<T> {
    // `&self`, not `self`!
    // `self`ではなく`&self`です！
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        // Implementation details
    }
}
```

It is therefore enough to send a shared reference to the client.\
We can't do that directly, though, because the reference would have to be `'static` and that's not the case.\
In a way, we need an "owned shared reference". It turns out that Rust has a type that fits the bill: `Arc`.

> よって、クライアントに共有参照を送信するだけで十分です。
> それを直接行うことはできませんが、その参照は`'static`である必要があり、それはその場合ではありません。
> ある意味で、「所有した共有参照」が必要です。それは、Rustがその要件を満たす型`Arc`を持っていることがわかります。

## `Arc` to the rescue（救うためのArc）

`Arc` stands for **atomic reference counting**.\
`Arc` wraps around a value and keeps track of how many references to the value exist.
When the last reference is dropped, the value is deallocated.\
The value wrapped in an `Arc` is immutable: you can only get shared references to it.

> `Arc`は**アトミックな参照カウンター**を表しています。
> `Arc`は値をラップして、値への参照がどれだけ存在するか追跡します。
> 最後の参照がドロップされたとき、その値はドロップされます。
> `Arc`内にラップされた値は不変です。その共有参照のみ得られます。

```rust
use std::sync::Arc;

let data: Arc<u32> = Arc::new(0);
let data_clone = Arc::clone(&data);

// `Arc<T>` implements `Deref<T>`, so can convert
// a `&Arc<T>` to a `&T` using deref coercion
// `Arc<T>`は`Deref<T>`を実装しているため、参照外し型強制を使用して`&Arc<T>`を`&T`に変換できます。
let data_ref: &u32 = &data;
```

If you're having a déjà vu moment, you're right: `Arc` sounds very similar to `Rc`, the reference-counted pointer we
introduced when talking about interior mutability. The difference is thread-safety: `Rc` is not `Send`, while `Arc` is.
It boils down to the way the reference count is implemented: `Rc` uses a "normal" integer, while `Arc` uses an
**atomic** integer, which can be safely shared and modified across threads.

> デジャブを感じた場合、それは正しいです。
> `Arc`は`Rc`にとてもにているように聞こえて、内部可変性について話していたときに導入した参照カウンターのポインターです。
> 違いはスレッドセーフです。`Rc`は`Send`ではなく、`Arc`は`Send`です。
> それは、参照カウンターを実装する方法を要約しています。
> `Rc`は「普通の」整数を使用している一方で、`Arc`は**アトミックな**整数を使用していて、それはスレッドをまたいで安全に共有と変更ができます。

## `Arc<Mutex<T>>`

If we pair `Arc` with `Mutex`, we finally get a type that:

- Can be sent between threads, because:
  - `Arc` is `Send` if `T` is `Send`, and
  - `Mutex` is `Send` if `T` is `Send`.
  - `T` is `Ticket`, which is `Send`.
- Can be cloned, because `Arc` is `Clone` no matter what `T` is.
  Cloning an `Arc` increments the reference count, the data is not copied.
- Can be used to modify the data it wraps, because `Arc` lets you get a shared
  reference to `Mutex<T>` which can in turn be used to acquire a lock.

> `Arc`と`Mutex`で組み合わせた場合、最終的に次の型が得られます。
>
> - スレッド間で送信できます。なぜなら・・・
>   - `T`が`Send`である場合、`Arc`は`Send`で・・・
>   - `T`が`Send`である場合、`Mutex`は`Send`です。
>   - `T`は`Ticket`で、それは`Send`です。
> - `T`に関わらず、`Arc`は`Clone`であるため、クローンできます。
>   `Arc`のクローンは、参照カウンターを増やし、そのデータはコピーされません。
> - それ（`Arc`）がラップしたデータを変更できます。
>   なぜなら、`Arc`は`Mutex<T>`への共有参照を得られるようにして、次にそれ（`Mutex<T>`）はロックを獲得するために使用されます。

We have all the pieces we need to implement the locking strategy for our ticket store.

> チケットストア用にロック戦略を実装するために必要なすべての要素が揃いました。

## Further reading（参考資料）

- We won't be covering the details of atomic operations in this course, but you can find more information
  [in the `std` documentation](https://doc.rust-lang.org/std/sync/atomic/index.html) as well as in the
  ["Rust atomics and locks" book](https://marabos.nl/atomics/).
