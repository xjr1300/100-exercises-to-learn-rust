# Ordering（順序）

By moving from a `Vec` to a `HashMap` we have improved the performance of our ticket management system,
and simplified our code in the process.\
It's not all roses, though. When iterating over a `Vec`-backed store, we could be sure that the tickets
would be returned in the order they were added.\
That's not the case with a `HashMap`: you can iterate over the tickets, but the order is random.

> `Vec`から`HashMap`に移動することにより、チケット管理システムの性能を改善して、その仮定でコードを簡素化しました。
> ただし、すべてがバラ色とは限りません。`Vec`に裏付けされたストアを反復操作するとき、チケットが追加された順番で返されることを確信できました。
> `HashMap`ではそれは当てはまりません。チケットを反復操作できますが、順序はランダムです。

We can recover a consistent ordering by switching from a `HashMap` to a `BTreeMap`.

> `HashMap`から`BTreeMap`に切り替えることで、順番の一貫性を回復できます。

## `BTreeMap`

A `BTreeMap` guarantees that entries are sorted by their keys.\
This is useful when you need to iterate over the entries in a specific order, or if you need to
perform range queries (e.g. "give me all tickets with an id between 10 and 20").

> `BTreeMap`は、エントリーがそれらのキーによって並べ替えされることを保証します。
> 特定の順番でエントリーを反復操作する必要があるとき、または、例えば、10と20の間のIDを持つすべてのチケットを取得するなど、範囲クエリを実行する必要があるとき、これは便利です。

Just like `HashMap`, you won't find trait bounds on the definition of `BTreeMap`.
But you'll find trait bounds on its methods. Let's look at `insert`:

> ちょうど`HashMap`と同様に、`BTreeMap`の定義にトレイト制約を見つけれないでしょう。
> しかし、そのメソッドにトレイト制約を見つけるでしょう。`insert`を確認しましょう。

```rust
// `K` and `V` stand for the key and value types, respectively,
// just like in `HashMap`.
// `K`と`V`は、ちょうど`HashMap`と同様に、それぞれキーと値の型を表します。
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // implementation
    }
}
```

`Hash` is no longer required. Instead, the key type must implement the `Ord` trait.

> `Hash`はもはや要求されません。代わりに、キー型は`Ord`トレイトを実装しなくてはなりませんん。

## `Ord`

The `Ord` trait is used to compare values.\
While `PartialEq` is used to compare for equality, `Ord` is used to compare for ordering.

> `Ord`トレイトは値を比較するために使用されます。
> `PartialEq`が等価性を比較するために使用される一方で、`Ord`は順序を比較するために使用されます。

It's defined in `std::cmp`:

> それは`std::cmp`で定義されています。

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

The `cmp` method returns an `Ordering` enum, which can be one
of `Less`, `Equal`, or `Greater`.\
`Ord` requires that two other traits are implemented: `Eq` and `PartialOrd`.

> `cmp`メソッドは`Ordering`列挙型を返し、それは`Less`、`Equal`、または`Greater`のいずれかになります。
> `Ord`は、`Eq`と`PartialOrd`の2つの他のトレイトが実装されていることを要求します。

## `PartialOrd`

`PartialOrd` is a weaker version of `Ord`, just like `PartialEq` is a weaker version of `Eq`.
You can see why by looking at its definition:

> `PartialOrd`は`Ord`の弱いバージョンで、ちょうど`PartialEq`が`Eq`の弱いバージョンであることと同じです。
> その定義を確認することで、理由がわかります。

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` returns an `Option`—it is not guaranteed that two values can
be compared.\
For example, `f32` doesn't implement `Ord` because `NaN` values are not comparable,
the same reason why `f32` doesn't implement `Eq`.

> `PartialOrd::partial_cmp`は`Option`を返します。それは、2つの値が比較できることを保証していません。
> 例えば、`f32`は、`NaN`値は比較可能でないため、`Ord`を実装しておらず、同じ理由で`f32`は`Eq`を実装していません。

## Implementing `Ord` and `PartialOrd`（OrdとPartialOrdの実装）

Both `Ord` and `PartialOrd` can be derived for your types:

> `Ord`と`PartialOrd`両方は、型から導出できます。

```rust
// You need to add `Eq` and `PartialEq` too,
// since `Ord` requires them.
// `Ord`が要求するため、`Eq`と`PartialEq`を追加する必要があります。
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

If you choose (or need) to implement them manually, be careful:

- `Ord` and `PartialOrd` must be consistent with `Eq` and `PartialEq`.
- `Ord` and `PartialOrd` must be consistent with each other.

> 手動でそれらを実装することを選択または必要がある場合、次に注意してください。
>
> - `Ord`と`PartialOrd`は、`Eq`と`PartialEq`と一貫性を持たなければなりません。
> - `Ord`と`PartialOrd`は、それぞれと一貫性を持たせなければなりません。
