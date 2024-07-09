# Structs（構造体）

We need to keep track of three pieces of information for each ticket:

- A title
- A description
- A status

> それぞれのチケットの3つの情報を追跡し続ける必要があります。
>
> - タイトル
> - 説明
> - 状態

We can start by using a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
to represent them. `String` is the type defined in Rust's standard library to represent
[UTF-8 encoded](https://en.wikipedia.org/wiki/UTF-8) text.

But how do we **combine** these three pieces of information into a single entity?

> それらを表現するために`String`を使用することから始めることができます。
> `String`は、`UTF-8エンコード`テキストを表現するために、Rustの標準ライブラリに定義された型です。
>
> しかし、どのようにこれら3つの情報を単一のエントリに**組み合わせる**のでしょうか？

## Defining a `struct`（`struct`の定義）

A `struct` defines a **new Rust type**.

> `struct`は、**新しいRustの型**を定義します。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

A struct is quite similar to what you would call a class or an object in other programming languages.

> 構造体は、他のプログラミング言語で暮らすまたはオブジェクトと呼ぶものにとても似ています。

## Defining fields（フィールドの定義）

The new type is built by combining other types as **fields**.\
Each field must have a name and a type, separated by a colon, `:`. If there are multiple fields, they are separated by a comma, `,`.

Fields don't have to be of the same type, as you can see in the `Configuration` struct below:

> 新しい型は、**フィールド**として他の型を組み合わせることによって構築されます。
> それぞれのフィールドは、コロン`:`で区切られた名前と型を持たなければなりません。
> 複数のフィールドがある場合、それらはカンマ`,`で区切られます。
>
> 下の`Configuration`構造体で確認できるように、フィールドは同じ型である必要はありません。

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## Instantiation（インスタンス化）

You can create an instance of a struct by specifying the values for each field:

> それぞれのフィールドの値を指定することで、構造体のインスタンスを作成できます。

```rust
// Syntax: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "Create a system that can manage tickets across a Kanban board".into(),
    status: "Open".into()
};
```

## Accessing fields（フィールドアクセス）

You can access the fields of a struct using the `.` operator:

> `.`演算子を使用して、構造体のフィールドにアクセスできます。

```rust
// Field access
let x = ticket.description;
```

## Methods（メソッド）

We can attach behaviour to our structs by defining **methods**.\
Using the `Ticket` struct as an example:

> **メソッド**を定義することで、構造体の振る舞いを取り付けできます。
> 例として、`Ticket`構造体を使用します。

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// Syntax:
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // Method body
//    }
// }
```

Methods are pretty similar to functions, with two key differences:

1. methods must be defined inside an **`impl` block**
2. methods may use `self` as their first parameter.
   `self` is a keyword and represents the instance of the struct the method is being called on.

> メソッドは、2つの主要な違いを持ちますが、関数ととても似ています。
>
> 1. メソッドは、**`impl`ブロック**内に定義されなければなりません。
> 2. メソッドは、それらの最初の引数として`self`を使用することができます。
>    `self`はキーワードであり、メソッドが呼ばれたインスタンスを表現します。

### `self`

If a method takes `self` as its first parameter, it can be called using the **method call syntax**:

> メソッドが最初の引数で`self`を受け取る場合、それは**メソッド呼び出し構文**を使用して呼び出すことができます。

```rust
// Method call syntax: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

This is the same calling syntax you used to perform saturating arithmetic operations on `u32` values
in [the previous chapter](../02_basic_calculator/09_saturating.md).

> これは、前のチャプターで飽和算術演算するために使用した呼び出し構文と同じです。

### Static methods（静的メソッド）

If a method doesn't take `self` as its first parameter, it's a **static method**.

> メソッドが最初の引数で`self`を受け取らない場合、それは**静的メソッド**です。

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` is a static method on `Configuration`
    // `default`は`Configuration`の静的メソッドです。
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

The only way to call a static method is by using the **function call syntax**:

> 静的メソッドを呼び出す唯一の方法は、**関数呼び出し構文**を使用することです。

```rust
// Function call syntax: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### Equivalence（等価）

You can use the function call syntax even for methods that take `self` as their first parameter:

> 最初の引数で`self`を受け取るメソッドに対しても、関数呼び出し構文を使用できます。

```rust
// Function call syntax: <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

The function call syntax makes it quite clear that `ticket` is being used as `self`, the first parameter of the method,
but it's definitely more verbose. Prefer the method call syntax when possible.

> 関数呼び出し構文は、`ticket`が、メソッドの最初の引数の`self`として使用されることをとても明確にしますが、間違いなくより冗長です。
> 可能であれば、メソッド呼び出し構文を使用してください。
