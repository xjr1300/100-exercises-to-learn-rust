# `HashMap`

Our implementation of `Index`/`IndexMut` is not ideal: we need to iterate over the entire
`Vec` to retrieve a ticket by id; the algorithmic complexity is `O(n)`, where
`n` is the number of tickets in the store.

> `Index`/`IndexMut`の実装は理想的ではありません。
> IDによってチケットを取得するために、`Vec`全体を反復操作する必要があります。

We can do better by using a different data structure for storing tickets: a `HashMap<K, V>`.

> チケットを格納するために異なるデータ構造である`HashMap<K, V>`を使用して、より良く行うことができます。

```rust
use std::collections::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
// 型推論は、明示的な型のシグネチャーを省略できるようにして、この例では`HashMap<String, String>`になります。
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` works with key-value pairs. It's generic over both: `K` is the generic
parameter for the key type, while `V` is the one for the value type.

> `HashMap`は、キーと値のペアで機能します。それは両方ともジェネリックです。
> `K`はキーの型のジェネリックパラメーターで、`V`は値の型のジェネリックパラメーターです。

The expected cost of insertions, retrievals and removals is **constant**, `O(1)`.
That sounds perfect for our usecase, doesn't it?

> 挿入、取得そして削除の期待されるコストは**一定**で、`O(1)`です。
> ユースケースにとって完璧ですよね？

## Key requirements（キーの要求事項）

There are no trait bounds on `HashMap`'s struct definition, but you'll find some
on its methods. Let's look at `insert`, for example:

> `HashMap`の構造体定義にはトレイト制約はありませんが、そのメソッドでいくつか確認できます。
> 例として、`insert`を確認しましょう。

```rust
// Slightly simplified
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

The key type must implement the `Eq` and `Hash` traits.\
Let's dig into those two.

> キーの型は、`Eq`と`Hash`トレイトを実装しなくてはなりません。
> それら2つの深堀りしましょう。

## `Hash`

A hashing function (or hasher) maps a potentially infinite set of a values (e.g.
all possible strings) to a bounded range (e.g. a `u64` value).\
There are many different hashing functions around, each with different properties
(speed, collision risk, reversibility, etc.).

> ハッシュ関数またはハッシャーは、例えばすべての考えられる文字列など潜在的に値の無限集合を、例えば`u64`値などの制限された範囲にマッピングします。
> 多くの異なるハッシュ関数があり、それぞれは異なる特徴を持っています（早さ、衝突リスク、可逆性など）。

A `HashMap`, as the name suggests, uses a hashing function behind the scene.
It hashes your key and then uses that hash to store/retrieve the associated value.
This strategy requires the key type must be hashable, hence the `Hash` trait bound on `K`.

> 名前が示唆する通り`HashMap`は、背後でハッシュ関数を使用しています。
> それはキーをハッシュ化して、関連した値を保存／取得するためにそのハッシュを使用します。
> この戦略は、キーの型がハッシュ可能でなくてはならず、そのため`Hash`トレイトが`K`に制約されています。

You can find the `Hash` trait in the `std::hash` module:

> `Hash`トレイトは`std::hash`モジュールにあります。

```rust
pub trait Hash {
    // Required method
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

You will rarely implement `Hash` manually. Most of the times you'll derive it:

> まれに手動で`Hash`を実装します。ほとんどの機会では、それを導出します。

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq`

`HashMap` must be able to compare keys for equality. This is particularly important
when dealing with hash collisions—i.e. when two different keys hash to the same value.

> `HashMap`は、等価性のためにキーを比較できなくてはなりません。
> 例えば2つの異なるキーが同じ値にハッシュ化され、ハッシュの衝突を扱うとき、これは特に重要になります。

You may wonder: isn't that what the `PartialEq` trait is for? Almost!\
`PartialEq` is not enough for `HashMap` because it doesn't guarantee reflexivity, i.e. `a == a` is always `true`.\
For example, floating point numbers (`f32` and `f64`) implement `PartialEq`,
but they don't satisfy the reflexivity property: `f32::NAN == f32::NAN` is `false`.\
Reflexivity is crucial for `HashMap` to work correctly: without it, you wouldn't be able to retrieve a value
from the map using the same key you used to insert it.

> 困惑しているかもしれません。そのために`PartialEq`トレイトがすることでは内のですか？ほぼそうです！
> 例えば、`a == a`が常に`true`であるなど、`PartialEq`は反射性を保証しないため、`HashMap`には十分ではありません。
> 例えば、浮動小数点数（`f32`と`f64`）は`PartialEq`を実装していますが、`f32::NAN == f32::NAN`がfalseになるように、それらは反射性の特徴を満たしていません。
> 反射性は、`HashMap`が正確に動作するために極めて重要です。
> それ無しで、挿入するために使用した同じキーを使用して、マップから値を取り出すことはできません。

The `Eq` trait extends `PartialEq` with the reflexivity property:

> `Eq`トレイトは、反射性の特性を持つ`PartialEq`を拡張します。

```rust
pub trait Eq: PartialEq {
    // No additional methods
}
```

It's a marker trait: it doesn't add any new methods, it's just a way for you to say to the compiler
that the equality logic implemented in `PartialEq` is reflexive.

> それはマーカートレイトです。それは任意の新しいメソッドを追加せず、`PartialEq`内に実装された等価性ロジックが反射性であることをコンパイラーに伝える単なる方法です。

You can derive `Eq` automatically when you derive `PartialEq`:

> `PartialEq`を導出するとき、`Eq`を自動で導出できます。

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq` and `Hash` are linked（EqとHashはリンクされている）

There is an implicit contract between `Eq` and `Hash`: if two keys are equal, their hashes must be equal too.
This is crucial for `HashMap` to work correctly. If you break this contract, you'll get nonsensical results
when using `HashMap`.

> 暗黙的な契約が`Eq`と`Hash`感にあります。2つのキーが等しい場合、それらのハッシュも等しくならなければなりません。
> これが、`HashMap`が正確に動作するために極めて重要です。
> この契約を破った場合、`HashMap`を使用したとき、無意味な結果が得られます。
