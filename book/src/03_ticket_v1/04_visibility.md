# Visibility（可視性）

When you start breaking down your code into multiple modules, you need to start thinking about **visibility**.
Visibility determines which regions of your code (or other people's code) can access a given entity,
be it a struct, a function, a field, etc.

> コードを複数のモジュールに分割し始めたとき、**可視性**について考え始めなければなりません。
> 可視性は、コードまたは他の人のコードのどの領域が、構造体、関数、フィールドなどの与えられたエンティティにアクセスできるかを決定します。

## Private by default（デフォルトでプライベート）

By default, everything in Rust is **private**.\
A private entity can only be accessed:

1. within the same module where it's defined, or
2. by one of its submodules

> デフォルトで、Rustのすべては**プライベート**です。
> プライベートなエンティティは、次のいずれかでのみアクセスできます。
>
> 1. それが定義された同じモジュール内で、または・・・
> 2. そのサブモジュールのいずれかによって

We've used this extensively in the previous exercises:

- `create_todo_ticket` worked (once you added a `use` statement) because `helpers` is a submodule of the crate root,
  where `Ticket` is defined. Therefore, `create_todo_ticket` can access `Ticket` without any issues even
  though `Ticket` is private.
- All our unit tests are defined in a submodule of the code they're testing, so they can access everything without
  restrictions.

> 前の演習でこれを広範囲に使用しました。
>
> - `create_todo_ticket`は、`helpers`が`Ticket`が定義されたクレートルートのサブモジュールのため、一旦`use`文を追加すると機能しました。
>   よって、`create_todo_ticket`は、`Ticket`がプライベートであるにも関わらず、問題なく`Ticket`にアクセスできます。
> - すべてのユニットテストは、ユニットテストがテストしているコード（`Ticket`）のサブモジュールに定義されたため、ユニットテストは制限無しですべてにアクセスできます。

## Visibility modifiers（可視性修飾子）

You can modify the default visibility of an entity using a **visibility modifier**.\
Some common visibility modifiers are:

- `pub`: makes the entity **public**, i.e. accessible from outside the module where it's defined, potentially from
  other crates.
- `pub(crate)`: makes the entity public within the same **crate**, but not outside of it.
- `pub(super)`: makes the entity public within the parent module.
- `pub(in path::to::module)`: makes the entity public within the specified module.

> **可視性修飾子**を使用して、エンティティのデフォルトの可視性を変更できます。
> いくつか一般的な可視性修飾子を次に示します。
>
> - `pub`: エンティティを**パブリック**にします。例えば、それが定義されたモジュールの外からアクセス可能で、他のクレートからも可能です。
> - `pub(crate)`: 同じ**クレート**内でエンティティをパブリックにしますが、そのクレートの外部からアクセスできません。
> - `pub(super)`: 親モジュール内でエンティティをパブリックにします。
> - `pub(in path::to::module)`: 指定されたモジュール内でエンティティをパブリックにします。

You can use these modifiers on modules, structs, functions, fields, etc.
For example:

> これらの修飾子を構造体、関数、フィールドなどに使用できます。
> 例えば・・・

```rust
pub struct Configuration {
    pub(crate) version: u32,
    active: bool,
}
```

`Configuration` is public, but you can only access the `version` field from within the same crate.
The `active` field, instead, is private and can only be accessed from within the same module or one of its submodules.

> `Configuration`はパブリックですが、同じクレート内でのみ`version`フィールドにアクセスできます。
> 代わりに`active`フィールドはプライベートで、同じモジュールかそのサブモジュール内でのみアクセスできます。
