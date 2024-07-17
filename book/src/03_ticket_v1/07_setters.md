# Mutable references（可変参照）

Your accessor methods should look like this now:

> 現在、アクセッサーメソッドは、次のようになります。

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

A sprinkle of `&` here and there did the trick!\
We now have a way to access the fields of a `Ticket` instance without consuming it in the process.
Let's see how we can enhance our `Ticket` struct with **setter methods** next.

> ここに`&`を振りかけることで、それらはうまくいきました！
> 現在、処理で`Ticket`インスタンスを消費しないで、`Ticket`インスタンスのフィールドにアクセスする方法を持つようになりました。
> 次に、**セッターメソッド**で`Ticket`構造体を強化する方法を確認しましょう。

## Setters（セッター）

Setter methods allow users to change the values of `Ticket`'s private fields while making sure that its invariants
are respected (i.e. you can't set a `Ticket`'s title to an empty string).

> セッターメソッドは、例えば、`Ticket`のタイトルに空の文字列を設定できないなど、`Ticket`の不変性を尊重することを確実にする一方で、`Ticket`のプライベートフィールドの値を変更できるようにします。

There are two common ways to implement setters in Rust:

- Taking `self` as input.
- Taking `&mut self` as input.

> Rustにおいてセッターを実装する2つの一般的な方法があります。
>
> - 入力として`self`を受け取る。
> - 入力として`&mut self`を受け取る。

### Taking `self` as input（入力としてselfを受け取る）

The first approach looks like this:

> 最初の方法は次のようになります。

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Validate the new title [...]
        // 新しいタイトルを検証します。
        self.title = new_title;
        self
    }
}
```

It takes ownership of `self`, changes the title, and returns the modified `Ticket` instance.\
This is how you'd use it:

> それは、`self`の所有権を取得して、タイトルを変更した後、変更された`Ticket`インスタンスを返します。
> 次はそれを使用する方法です。

```rust
let ticket = Ticket::new("Title".into(), "Description".into(), "To-Do".into());
let ticket = ticket.set_title("New title".into());
```

Since `set_title` takes ownership of `self` (i.e. it **consumes it**), we need to reassign the result to a variable.
In the example above we take advantage of **variable shadowing** to reuse the same variable name: when
you declare a new variable with the same name as an existing one, the new variable **shadows** the old one. This
is a common pattern in Rust code.

> `set_title`が`self`の所有権を取得して、**それを消費する**ため、変数に結果を再割り当てする必要があります。
> 上記例において、変数の名前を再利用する_変数のシャドーイング_を利用しています。
> 存在する同じ名前で新しい変数を宣言したとき、新しい変数は古いものを**隠します**。
> Rustのコードで、これは一般的なパターンです。

`self`-setters work quite nicely when you need to change multiple fields at once: you can chain multiple calls together!

> 一度に複数のフィールドを変更する必要があるときも、`self`セッターはとても良く機能します。
> 複数の呼び出しを一緒に繋げれます。

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Taking `&mut self` as input（入力として&mut selfを受け取る）

The second approach to setters, using `&mut self`, looks like this instead:

> `&mut self`を使用するセッターの2つ目の方法は、次のようになります。

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Validate the new title [...]
        // 新しいタイトルを検証します。

        self.title = new_title;
    }
}
```

This time the method takes a mutable reference to `self` as input, changes the title, and that's it.
Nothing is returned.

> 今回、メソッドは入力として`self`への可変参照を受け取り、タイトルを変更して、それだけです。
> 何も返されません。

You'd use it like this:

> 次のようにそれを使用します。

```rust
let mut ticket = Ticket::new("Title".into(), "Description".into(), "To-Do".into());
ticket.set_title("New title".into());

// Use the modified ticket
// 変更されたチケットを使用します。
```

Ownership stays with the caller, so the original `ticket` variable is still valid. We don't need to reassign the result.
We need to mark `ticket` as mutable though, because we're taking a mutable reference to it.

> 所有権は呼び出し元に留まるため、オリジナルの`ticket`変数はまだ有効です。
> 結果を再割り当てする必要はありません。
> しかし、その可変参照を得ているため、可変として`ticket`をマークする必要があります。

`&mut`-setters have a downside: you can't chain multiple calls together.
Since they don't return the modified `Ticket` instance, you can't call another setter on the result of the first one.
You have to call each setter separately:

> `&mut`セッターは欠点があります。複数の呼び出しを一緒に繋げれません。
> 変更された`ticket`インスタンスを返さないため、最初の結果で他のセッターを呼び出しできません。

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
