# Concise branching（簡潔なブランチ分かれ）

Your solution to the previous exercise probably looks like this:

> 前の演習の解答は、おそらく次のようになっているでしょう。

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!("Only `In-Progress` tickets can be assigned to someone")
            }
        }
    }
}
```

You only care about the `Status::InProgress` variant.
Do you really need to match on all the other variants?

> `Status::InProgress`バリアントについて気にかけるだけです。
> 本当にすべての他のバリアントにマッチする必要があるのでしょうか？

New constructs to the rescue!

> 新しい構築物が救援します。

## `if let`

The `if let` construct allows you to match on a single variant of an enum,
without having to handle all the other variants.

> `if let`構築は、他のバリアントすべてを処理することなく、列挙型の1つのバリアントにマッチさせます。

Here's how you can use `if let` to simplify the `assigned_to` method:

> `assigned_to`メソッドを単純化するために、`if let`を使用する方法をここに示します。

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!("Only `In-Progress` tickets can be assigned to someone");
        }
    }
}
```

## `let/else`

If the `else` branch is meant to return early (a panic counts as returning early!),
you can use the `let/else` construct:

> `else`ブランチが早期リターン（パニックを早期リターンとして考慮します）を意図する場合、`let/else`構築を使用できます。

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!("Only `In-Progress` tickets can be assigned to someone");
        };
        assigned_to
    }
}
```

It allows you to assign the destructured variable without incurring
any "right drift", i.e. the variable is assigned at the same indentation level
as the code that precedes it.

> それは、「右ドリフト」を招くことなく、分割された変数を割り当てさせます。
> 例えば、変数はそれに先立つコードと同じインデントレベルで割り当てられます。

## Style（スタイル）

Both `if let` and `let/else` are idiomatic Rust constructs.\
Use them as you see fit to improve the readability of your code,
but don't overdo it: `match` is always there when you need it.

> `if let`と`let/else`の両方は、Rustの慣用的な構築物です。
> それらの使用は、コードの可読性を改善するために適切のように見えますが、それを過度に使用しないでください。
> `match`は常に必要なときに使用できます。
