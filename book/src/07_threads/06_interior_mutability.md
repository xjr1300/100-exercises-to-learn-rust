# Interior mutability（内部可変性）

Let's take a moment to reason about the signature of `Sender`'s `send`:

> `Sender`の`send`のシグネチャーについて理由を考える瞬間です。

```rust
impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // [...]
    }
}
```

`send` takes `&self` as its argument.\
But it's clearly causing a mutation: it's adding a new message to the channel.
What's even more interesting is that `Sender` is cloneable: we can have multiple instances of `Sender`
trying to modify the channel state **at the same time**, from different threads.

> `send`はその引数として`&self`を受け取ります。
> しかし、それは明らかに変異を起こします。それは、チャネルに新しいメッセージを追加するからです。
> より興味深いことは、`Sender`がクローン可能なことです。
> 異なるスレッドで**同時に**チャネルの状態を変更することを試みる、複数の`Sender`インスタンスを持てます。

That's the key property we are using to build this client-server architecture. But why does it work?
Doesn't it violate Rust's rules about borrowing? How are we performing mutations via an _immutable_ reference?

> それは、このクライアントサーバーアーキテクチャーを構築するために使用した重要な特徴です。
> しかし、それはなぜ機能するのでしょうか？
> それは、Rustの借用ルールを破っていませんか？
> _不変_ 参照を介して、どのように変異を行っているのでしょうか？

## Shared rather than immutable references（不変参照ではなく共有）

When we introduced the borrow-checker, we named the two types of references we can have in Rust:

- immutable references (`&T`)
- mutable references (`&mut T`)

> 借用チェッカーを紹介したとき、Rustで持てる2種類の参照を名付けました。
>
> - 不変参照（`&T`）
> - 可変参照（`&mut T`）

It would have been more accurate to name them:

- shared references (`&T`)
- exclusive references (`&mut T`)

> それらはより正確な名前が付けられています。
>
> - 共有参照（`&T`）
> - 排他参照（`&mut T`）

Immutable/mutable is a mental model that works for the vast majority of cases, and it's a great one to get started
with Rust. But it's not the whole story, as you've just seen: `&T` doesn't actually guarantee that the data it
points to is immutable.\
Don't worry, though: Rust is still keeping its promises.
It's just that the terms are a bit more nuanced than they might seem at first.

> 不変／可変は、ほとんどの大半のケースで機能する価値観（メンタルモデル）であり、Rustを始める最適なものです。
> しかし、それは全体のストーリーではありません。ちょうど見たように、`&T`はそれが指し示すデータが不変であることを、実際に保証していません。
> ただし、心配しないでください。Rustはまだその約束を守っています。
> それらを最初に見たときよりも、用語が少しより微妙になっただけです。

## `UnsafeCell`

Whenever a type allows you to mutate data through a shared reference, you're dealing with **interior mutability**.

> 共有参照を介して型がデータを変更できるようにしたときはいつでも、**内部可変性**を扱っています。

By default, the Rust compiler assumes that shared references are immutable. It **optimises your code** based on that assumption.\
The compiler can reorder operations, cache values, and do all sorts of magic to make your code faster.

> デフォルトで、Rustコンパイラーは、共有参照が不変であると仮定します。
> コンパイラーはその仮定に基づいて、**コードを最適化**します。
> コンパイラーは、操作を再配置、値をキャッシュ、そして、コードを早くするためにあらゆる種類の魔法をかけることができます。

You can tell the compiler "No, this shared reference is actually mutable" by wrapping the data in an `UnsafeCell`.\
Every time you see a type that allows interior mutability, you can be certain that `UnsafeCell` is involved,
either directly or indirectly.\
Using `UnsafeCell`, raw pointers and `unsafe` code, you can mutate data through shared references.

> `UnsafeCell`内にデータをラップすることで、「いいえ、この共有参照は、実際には変更可能です」とコンパイラーに伝えることができます。
> 内部可変性を許可する型を見るたびに、`UnsafeCell`が関与していることを確信できます。
> `UnsafeCell`、生ポインターそして`unsafe`コードを使用して、共有参照を介してデータを変更できます。

Let's be clear, though: `UnsafeCell` isn't a magic wand that allows you to ignore the borrow-checker!\
`unsafe` code is still subject to Rust's rules about borrowing and aliasing.
It's an (advanced) tool that you can leverage to build **safe abstractions** whose safety can't be directly expressed
in Rust's type system. Whenever you use the `unsafe` keyword you're telling the compiler:
"I know what I'm doing, I won't violate your invariants, trust me."

> ただし、明確にしましょう。`UnsafeCell`は、借用チェッカーを無視させる魔法の杖ではありません！
> `unsafe`コードは、まだ借用とエイリアスに関するRustのルールの対象です。
> それは、Rustの型システム内で、安全性を直接表現できない**安全な抽象化**を構築するために利用できる高度なツールです。
> `unsafe`キーワードを使用するときはいつでも、コンパイラーに次を伝えています。
> 「私は何をしているか理解しています。あなたの不変条件を破るつもりはありません。信じてください。」

Every time you call an `unsafe` function, there will be documentation explaining its **safety preconditions**:
under what circumstances it's safe to execute its `unsafe` block. You can find the ones for `UnsafeCell`
[in `std`'s documentation](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html).

> `unsafe`関数を呼び出すたびに、その`unsafe`ブロックを実行する安全な環境の下で、その**安全性の前提条件**を説明するドキュメントがあります。
> 標準ライブラリの`UnsafeCell`で見つけることができます。

We won't be using `UnsafeCell` directly in this course, nor will we be writing `unsafe` code.
But it's important to know that it's there, why it exists and how it relates to the types you use
every day in Rust.

> このコースにおいて、`UnsafeCell`を直接使用しませんし、`unsafe`コードを書くこともありません。
> しかし、Rust内になぜそれが存在して、それがどのように毎日使用する型に関連するのかを理解することは重要です。

## Key examples（鍵となる例）

Let's go through a couple of important `std` types that leverage interior mutability.\
These are types that you'll encounter somewhat often in Rust code, especially if you peek under the hood of
some the libraries you use.

> 内部可変性を利用する重要な`std`の型の組を確認しましょう。
> 特に使用するライブラリの内部を覗いた場合、Rustコードでかなり頻繁に遭遇する型があります。

### Reference counting（参照カウンター）

`Rc` is a reference-counted pointer.\
It wraps around a value and keeps track of how many references to the value exist.
When the last reference is dropped, the value is deallocated.\
The value wrapped in an `Rc` is immutable: you can only get shared references to it.

> `Rc`は参照をカウントするポインターです。
> それは値をラップして、その値への参照が存在する数を追跡します。
> 最後の参照がドロップされたとき、その値は解放されます。
> `Rc`にラップされた値は不変です。それから共有参照のみ得られます。

```rust
use std::rc::Rc;

let a: Rc<String> = Rc::new("My string".to_string());
// Only one reference to the string data exists.
// その文字列データへの参照が唯一存在します。
assert_eq!(Rc::strong_count(&a), 1);

// When we call `clone`, the string data is not copied!
// Instead, the reference count for `Rc` is incremented.
// `clone`を呼び出したとき、文字列データはコピーされません！
// 代わりに、`Rc`の参照カウンターが増加されます。
let b = Rc::clone(&a);
assert_eq!(Rc::strong_count(&a), 2);
assert_eq!(Rc::strong_count(&b), 2);
// ^ Both `a` and `b` point to the same string data
//   and share the same reference counter.
// `a`と`b`両方は、同じ文字列データを指し示して、同じ参照カウンターを共有しています。
```

`Rc` uses `UnsafeCell` internally to allow shared references to increment and decrement the reference count.

> `Rc`は、共有参照が参照の数を増やしたり減らしたりするために、内部で`UnsafeCell`を使用しています。

> 上記例の`Rc`型の変数`a`と`b`は`mut`でないため不変である。
> しかし、`a`と`b`が共有する参照カウンターを変更するために、内部で`UnsafeCell`を使用する必要がある。

### `RefCell`

`RefCell` is one of the most common examples of interior mutability in Rust.
It allows you to mutate the value wrapped in a `RefCell` even if you only have an
immutable reference to the `RefCell` itself.

> `RefCell`は、Rustにおいて内部可変性の最も一般的な例の1つです。
> それは、`RefCell`自身への不変参照を持っていた場合でも、`RefCell`内にラップした値を変更させます。

This is done via **runtime borrow checking**.
The `RefCell` keeps track of the number (and type) of references to the value it contains at runtime.
If you try to borrow the value mutably while it's already borrowed immutably,
the program will panic, ensuring that Rust's borrowing rules are always enforced.

> これは、**ランタイム借用チェッカー**を介して行われます。
> `RefCell`は、ランタイムでそれが含む値への参照の数（と型）を追跡します。
> 値がすでに不変で借用されている間に、値を可変で借用することを試みた場合、プログラムはパニックして、Rustの借用ルールが常に強制されることを保証します。

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow(); // Immutable borrow（不変借用）
let z = x.borrow_mut(); // Panics! There is an active immutable borrow.（パニックします！アクティブな不変借用があります）
```
