# Derive macros（導出マクロ）

Implementing `PartialEq` for `Ticket` was a bit tedious, wasn't it?
You had to manually compare each field of the struct.

> `Ticket`の`PartialEq`の実装は少し退屈ではないですか？
> 構造体のそれぞれのフィールドを手動で比較しなくてはなりませんでした。

## Destructuring syntax（分割構文）

Furthermore, the implementation is brittle: if the struct definition changes
(e.g. a new field is added), you have to remember to update the `PartialEq` implementation.

> さらに、その実装は壊れやすいです。
> 新しいフィールドが追加されるなど、構造体の定義が変更した場合、`PartialEq`の実装を更新することを忘れないようにしなくてはなりません。

You can mitigate the risk by **destructuring** the struct into its fields:

> 構造体をそのフィールドに**分割**することで、そのリスクを軽減できます。

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

If the definition of `Ticket` changes, the compiler will error out, complaining that your
destructuring is no longer exhaustive.\
You can also rename struct fields, to avoid variable shadowing:

> `Ticket`の定義が変更された場合、コンパイラーは、分割がもはや網羅的でないとしてエラーを出力します。
> また、変数のシャドーイングを防ぐために、構造体のフィールドの名前を変更できます。

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Destructuring is a useful pattern to have in your toolkit, but
there's an even more convenient way to do this: **derive macros**.

> 分割はツールキット内にあると便利なパターンですが、実装するために、より便利な方法があります。
> それは**導出マクロ**です。

## Macros（マクロ）

You've already encountered a few macros in past exercises:

- `assert_eq!` and `assert!`, in the test cases
- `println!`, to print to the console

> 過去の演習ですでにいくつかのマクロに遭遇しています。
>
> - テストケース内の`assert_eq!`と`assert!`
> - コンソールに出力するための`println!`

Rust macros are **code generators**.\
They generate new Rust code based on the input you provide, and that generated code is then compiled alongside
the rest of your program. Some macros are built into Rust's standard library, but you can also
write your own. We won't be creating our own macro in this course, but you can find some useful
pointers in the ["Further reading" section](#further-reading参考資料)

> Rustのマクロは**コードジェネレーター**です。
> それらは、提供した入力に基づいて新しいRustコードを生成して、生成されたコードはプログラムの残りと一緒にコンパイルされます。
> いくつかのマクロは、Rust標準ライブラリ内に組み込まれていますが、独自に記述することもできます。
> このコースで独自のマクロを作成するつもりはありませんが、参考資料の節にいくつかの有益な指針を見つけれます。

### Inspection（検査）

Some IDEs let you expand a macro to inspect the generated code. If that's not possible, you can use
[`cargo-expand`](https://github.com/dtolnay/cargo-expand).

> 一部のIDEは、生成されるコードを検査するためにマクロを展開します。
> それが不可能な場合、`cargo-expand`を使用できます。

### Derive macros（導出マクロ）

A **derive macro** is a particular flavour of Rust macro. It is specified as an **attribute** on top of a struct.

> **導出マクロ**は、Rustマクロの特別な風味です。
> それは、構造体の上に**属性**として指定されます。

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros are used to automate the implementation of common (and "obvious") traits for custom types.
In the example above, the `PartialEq` trait is automatically implemented for `Ticket`.
If you expand the macro, you'll see that the generated code is functionally equivalent to the one you wrote manually,
although a bit more cumbersome to read:

> 導出マクロは、カスタム型に一般的なトレイトの実装を自動化するために使用されます。
> 上記例において、`PartialEq`トレイトは`Ticket`に自動で実装されます。
> マクロを展開した場合、読むのがより少し煩わしいですが、手動で記述したものと機能的に等価なコードが生成されることを確認できます。

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title && self.description == other.description
            && self.status == other.status
    }
}
```

The compiler will nudge you to derive traits when possible.

> コンパイラーは、可能な場合にトレイトを導出することを推奨します。

## Further reading（参考資料）

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
