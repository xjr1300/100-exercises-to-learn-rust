# Encapsulation（カプセル化）

Now that we have a basic understanding of modules and visibility, let's circle back to **encapsulation**.\
Encapsulation is the practice of hiding the internal representation of an object. It is most commonly
used to enforce some **invariants** on the object's state.

> 現在、モジュールと可視性の基本的な理解があるため、**カプセル化**について再度考えてみましょう。
> カプセル化は、オブジェクトの内部表現を隠す実践です。
> それは、オブジェクトの状態に、何らかの**不変**に強制するために、最も一般的に使用されます。

Going back to our `Ticket` struct:

> `Ticket`構造体に戻りましょう。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

If all fields are made public, there is no encapsulation.\
You must assume that the fields can be modified at any time, set to any value that's allowed by
their type. You can't rule out that a ticket might have an empty title or a status
that doesn't make sense.

> すべてのフィールドをパブリックにする場合、カプセル化はありません。
> いつでもフィールドが変更され、それらの型に従って任意な値を設定されることを想定しなければなりません。
> チケットが、意味のない空のタイトルや状態を持つかもしれず、それを除外できません。

To enforce stricter rules, we must keep the fields private[^newtype].
We can then provide public methods to interact with a `Ticket` instance.
Those public methods will have the responsibility of upholding our invariants (e.g. a title must not be empty).

> 厳密なルールを強制するために、フィールドをプライベートに維持しなければなりません。
> そして、`Ticket`インスタンスと相互作用するパブリックなメソッドを提供できます。
> それらパブリックメソッドは、例えばタイトルは空であってはならないなどの、不変を維持する責任をもちます。

If all fields are private, it is no longer possible to create a `Ticket` instance directly using the struct
instantiation syntax:

> すべてのフィールドがプライベートの場合、構造体インスタンス化構文を使用して、直接`Ticket`インスタンスを作成できなくなります。

```rust
// This won't work!
// （`Ticket`構造体のすべてのフィールドがプライベートであるため）次は機能しません！
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "Create a system that can manage tickets across a Kanban board".into(),
    status: "Open".into()
};
```

You've seen this in action in the previous exercise on visibility.\
We now need to provide one or more public **constructors**—i.e. static methods or functions that can be used
from outside the module to create a new instance of the struct.\
Luckily enough we already have one: `Ticket::new`, as implemented in [a previous exercise](02_validation.md).

> 可視性の前の演習内で実際にこれを確認しました。
> 現在、例えば構造体の新しいインスタンスを作成するために、モジュールの外側から使用される静的メソッドまたは関数など、1つ以上のパブリックな**コンストラクター**を提供する必要があります。
> とても幸いなことに、前の演習で実装した`Ticket::new`がすでにあります。

## Accessor methods（アクセッサーメソッド）

In summary:

- All `Ticket` fields are private
- We provide a public constructor, `Ticket::new`, that enforces our validation rules on creation

> 要約すると:
>
> - すべての`Ticket`フィールドはプライベートです。
> - 作成時の検証ルールを強制するパブリックコンストラクターである`Ticket::new`を提供します。

That's a good start, but it's not enough: apart from creating a `Ticket`, we also need to interact with it.
But how can we access the fields if they're private?

> これは良いスタートですが、十分ではありません。`Ticket`の作成とは別に、チケットと相互作用する必要があります。
> しかし、フィールドがプライベートな場合、どのようにアクセスできるのでしょうか？

We need to provide **accessor methods**.\
Accessor methods are public methods that allow you to read the value of a private field (or fields) of a struct.

> **アクセッサーメソッド**を提供する必要があります。
> アクセッサーメソッドは、構造体のプライベートなフィールドまたは複数のフィールドの値を読み取れるパブリックメソッドです。

Rust doesn't have a built-in way to generate accessor methods for you, like some other languages do.
You have to write them yourself—they're just regular methods.

> Rustは、他の言語のように、アクセッサーメソッドを生成する組み込みの仕組みがありません。
> アクセッサーメソッドを自身で記述しなくてはなりません。アクセッサーメソッドは単なる普通のメソッドです。

[^newtype]: Or refine their type, a technique we'll explore [later on](../05_ticket_v2/15_outro.md).
または、型を洗練する後で探求するテクニックです。
