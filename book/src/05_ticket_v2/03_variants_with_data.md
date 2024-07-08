# Variants can hold data（バリアントはデータを保持できる）

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.

> `Status`列挙型は、通常**Cスタイルの列挙型**と呼ばれます。
> それぞれのバリアントは1つのラベルで、すこし名前付きの定数のようです。
> C、C++、Java、C#、Pythonなどのような多くのプログラミング言語で、この種類の列挙型を見つけれます。

Rust enums can go further though. We can **attach data to each variant**.

> しかし、Rustの列挙型は進んでいます。**それぞれのバリアントにデータを付けれます。**。

## Variants（バリアント）

Let's say that we want to store the name of the person who's currently working on a ticket.\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
We can model this by attaching a `String` field to the `InProgress` variant:

> 現在チケットを処理している人の名前を保存したいとします。
> この情報は、チケットが進行中の場合のみ持てるとします。
> これは、未完了のチケットまたは完了したチケットにはありません。
> `InProgress`バリアントに`String`フィールドを付けることで、これをモデル化できます。

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.

> `InProgress`は、現在、**構造体のようなバリアント**です。
> 実際、構文は、構造体を定義するときに使用したモノを反映しています。ちょうどそれはバリアントとして列挙型の内部を「インライン化」しています。

## Accessing variant data（バリアントのデータにアクセスする）

If we try to access `assigned_to` on a `Status` instance,

> `Status`インスタンスの`assigned_to`にアクセスすることを試みた場合、

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.assigned_to);
```

the compiler will stop us:

> コンパイラは止めるでしょう。

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` is **variant-specific**, it's not available on all `Status` instances.\
To access `assigned_to`, we need to use **pattern matching**:

> `assigned_to`は**バリアント固有**です。それはすべての`Status`インスタンスで利用できません。
> `assigned_to`にアクセスするために、**パターンマッチング**を使用する必要があります。

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("Done");
    }
}
```

## Bindings（バインディング）

In the match pattern `Status::InProgress { assigned_to }`, `assigned_to` is a **binding**.\
We're **destructuring** the `Status::InProgress` variant and binding the `assigned_to` field to
a new variable, also named `assigned_to`.\
If we wanted, we could bind the field to a different variable name:

> `Status::InProgress { assigned_to }`のマッチングパターンにおいて、`assigned_to`は**バインディング**です。
> `Status::InProgress`バリアントを**分割して**、新しい`assigned_to`変数に`assigned_to`フィールドをバインディングしています。
> 必要であれば、フィールドを異なる変数名にバインドできます。

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("Done");
    }
}
```
