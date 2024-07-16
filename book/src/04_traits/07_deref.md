# `Deref` trait（Derefトレイト）

In the previous exercise you didn't have to do much, did you?

> 前の演習において、あまり多くのことをする必要はありませんでしたよね？

Changing

> 変更前

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```

to

> 変更後

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```

was all you needed to do to get the code to compile and the tests to pass.
Some alarm bells should be ringing in your head though.

> コンパイルできるようにコードを得て、テストにパスするために必要なことは、これだけでした。

## It shouldn't work, but it does（それは機能素べきではないが、動作します）

Let's review the facts:

- `self.title` is a `String`
- `&self.title` is, therefore, a `&String`
- The output of the (modified) `title` method is `&str`

> 事実を振り返りましょう。
>
> - `self.title`は`String`
> - `&self.title`は、よって`&String`
> - 修正した`title`メソッドの出力は`&str`

You would expect a compiler error, wouldn't you? `Expected &String, found &str` or something similar.
Instead, it just works. **Why**?

> コンパイルエラーを期待したのではないでしょうか？`&Stringを期待しましたが、&strを見つけました`または何か同様なものを。
> 代わりに、それは単純に機能します。**なぜでしょうか**？

## `Deref` to the rescue（`Deref`が救出します）

The `Deref` trait is the mechanism behind the language feature known as [**deref coercion**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion).\
The trait is defined in the standard library, in the `std::ops` module:

> `Deref`トレイトは、**参照外し型強制**として知られる言語機能の背後にあるメカニズムです。
> そのトレイトは、標準ライブラリの`std::ops`モジュールで定義されています。

```rust
// I've slightly simplified the definition for now.
// We'll see the full definition later on.
// ここでは、定義を少し簡略化しています。
// 後で完全な定義を確認するつもりです。
pub trait Deref {
    type Target;

    fn deref(&self) -> &Self::Target;
}
```

`type Target` is an **associated type**.\
It's a placeholder for a concrete type that must be specified when the trait is implemented.

> `type Target`は**関連型**です。
> それは、トレイトが実装されたときに指定される必要がある具象型のプレースホルダーです。

## Deref coercion（参照外し型強制）

By implementing `Deref<Target = U>` for a type `T` you're telling the compiler that `&T` and `&U` are
somewhat interchangeable.\
In particular, you get the following behavior:

- References to `T` are implicitly converted into references to `U` (i.e. `&T` becomes `&U`)
- You can call on `&T` all the methods defined on `U` that take `&self` as input.

> 型`T`に対して`Deref<Target = U>`を実装することにより、`&T`と`&U`がある程度交換可能であることをコンパイラに伝えます。
> 特に、次のような振る舞いを得られます。
>
> - `T`への参照は、暗黙的に`U`への参照に変換されます（つまり、`&T`は`&U`になります）。
> - 入力として`&self`を受け取る`U`で定義されたすべてのメソッドを`&T`で呼び出すことができます。

There is one more thing around the dereference operator, `*`, but we don't need it yet (see `std`'s docs
if you're curious).

> 参照外し演算子`*`のまわりには、もう1つのことがありますが、まだそれは必要ありません（好奇心が強いのであれば、`std`のドキュメントを参照してください）。

## `String` implements `Deref`（StringはDerefを実装しています）

`String` implements `Deref` with `Target = str`:

> `String`は、`Target = str`を持つ`Deref`を実装しています。

```rust
impl Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        // [...]
    }
}
```

Thanks to this implementation and deref coercion, a `&String` is automatically converted into a `&str` when needed.

> この実装と参照外し型強制のおかげで、必要なときに`&String`は自動的に`&str`に変換されます。

## Don't abuse deref coercion（参照外し型強制を乱用しない）

Deref coercion is a powerful feature, but it can lead to confusion.\
Automatically converting types can make the code harder to read and understand. If a method with the same name
is defined on both `T` and `U`, which one will be called?

> 参照外し型強制は強力な機能ですが、混乱を招く可能性があります。
> 型の自動変換は、コードを読むことと理解することを難しくする可能性があります。
> `T`と`U`の両方に定義された同じ名前のメソッドがある場合、どちらが呼び出されるでしょうか？

We'll examine later in the course the "safest" use cases for deref coercion: smart pointers.

> このコースの後半で、参照外し型強制の「最も安全な」ユースケースを調査します。それはスマートポインターです。
