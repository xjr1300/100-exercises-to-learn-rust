# Validation（検証）

Let's go back to our ticket definition:

> チケットの定義に戻りましょう。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

We are using "raw" types for the fields of our `Ticket` struct.
This means that users can create a ticket with an empty title, a suuuuuuuper long description or
a nonsensical status (e.g. "Funny").\
We can do better than that!

> `Ticket`構造体のフィールドに「生」の型を使用しています。
> これは、空のタイトル、とても長い説明、または意味のない状態（例えば、「面白い」）でチケットを作成できることを意味します。
> もっと良い方法があります！

## Further reading（参考資料）

- Check out [`String`'s documentation](https://doc.rust-lang.org/std/string/struct.String.html)
  for a thorough overview of the methods it provides. You'll need it for the exercise!

> - `String`が提供するメソッドの概要を確認するために、`String`のドキュメントを参照してください。
>   この演習にはそれが必要です。
