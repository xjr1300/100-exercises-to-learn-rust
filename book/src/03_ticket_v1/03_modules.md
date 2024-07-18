# Modules（モジュール）

The `new` method you've just defined is trying to enforce some **constraints** on the field values for `Ticket`.
But are those invariants really enforced? What prevents a developer from creating a `Ticket`
without going through `Ticket::new`?

> ちょうど定義した`new`メソッドは、`Ticket`のフィールドの値に何らかの**制約**を強制することを試みます。
> しかし、それらの不変性は本当に強制されるのでしょうか？
> `Ticket::new`を介することなく、開発者が`Ticket`を作成することを防ぐのは何でしょうか？

To get proper **encapsulation** you need to become familiar with two new concepts: **visibility** and **modules**.
Let's start with modules.

> 適切な**カプセル化**を得るために、**可視性**と**モジュール**という2つの新しい概念に慣れる必要があります。
> モジュールから始めましょう。

## What is a module?（モジュールとは？）

In Rust a **module** is a way to group related code together, under a common namespace (i.e. the module's name).\
You've already seen modules in action: the unit tests that verify the correctness of your code are defined in a
different module, named `tests`.

> Rustにおいて、**モジュール**は、例えばモジュールの名前など、一般的な名前空間の下で、関連したコード同士でグループを作成する方法です。
> 実際にすでにモジュールを確認しています。コードの正確性を検証するユニットテストは、`tests`と名付けられた異なるモジュールに定義されています。

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## Inline modules（インラインモジュール）

The `tests` module above is an example of an **inline module**: the module declaration (`mod tests`) and the module
contents (the stuff inside `{ ... }`) are next to each other.

> 上記`tests`モジュールは、**インラインモジュール**の例で、そのモジュールの定義（`mod tests`）とそのモジュールのコンテンツ（`{ ... }`の内部のもの）は隣り合っています。

## Module tree（モジュールツリー）

Modules can be nested, forming a **tree** structure.\
The root of the tree is the **crate** itself, which is the top-level module that contains all the other modules.
For a library crate, the root module is usually `src/lib.rs` (unless its location has been customized).
The root module is also known as the **crate root**.

> モジュールはネストでき、**木**構造を形成します。
> 木のルートは**クレート**それ自身で、それはすべての他のモジュールを含む最上位モジュールです。
> ライブラリクレートの場合、場所がカスタマイズされていない限り、ルートモジュールは通常`src/lib.rs`です。
> また、ルートモジュールは、**クレートルート**としても知られています。

The crate root can have submodules, which in turn can have their own submodules, and so on.

> クレートルートはサブモジュールを持て、サブモジュールは自身のサブモジュールを持つことができ、それが続きます。

## External modules and the filesystem（外部モジュールとファイルシステム）

Inline modules are useful for small pieces of code, but as your project grows you'll want to split your code into
multiple files. In the parent module, you declare the existence of a submodule using the `mod` keyword.

> インラインモジュールは少量のコードで便利ですが、プロジェクトが成長するにつれて、コードを複数のファイルに分割したいと考えるでしょう。
> 親モジュールで、`mod`キーワードを使用して、サブモジュールの存在を宣言します。

```rust
mod dog;
```

`cargo`, Rust's build tool, is then in charge of finding the file that contains
the module implementation.\
If your module is declared in the root of your crate (e.g. `src/lib.rs` or `src/main.rs`),
`cargo` expects the file to be named either:

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

> Rustのビルドツールである`cargo`は、モジュール実装を含むファイルを検索する責任があります。
> `src/lib.rs`または`src/main.rs`など、モジュールがクレートのルートに宣言されている場合、`cargo`はファイルの名前が次のいずれかであることを期待します。
>
> - `src/<module_name>.rs`
> - `src/<module_name>/mod.rs`

If your module is a submodule of another module, the file should be named:

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

> モジュールが他のモジュールのサブモジュールの場合、ファイルは次の通り名付けられるべきです。
>
> - `[..]/<parent_module>/<module_name>.rs`
> - `[..]/<parent_module>/<module_name>/mod.rs`

E.g. `src/animals/dog.rs` or `src/animals/dog/mod.rs` if `dog` is a submodule of `animals`.

> 例えば、`dog`が`animals`のサブモジュールの場合、`src/animals/dog.rs`または`src/animals/dog/mod.rs`です。

Your IDE might help you create these files automatically when you declare a new module using the `mod` keyword.

> IDEは、`mod`キーワードを使用して新しいモジュールを宣言したとき、自動的にこれらのファイルを作成することを支援するかもしれません。

## Item paths and `use` statements（アイテムパスとuse文）

You can access items defined in the same module without any special syntax. You just use their name.

> 特別な構文なしで、同じモジュール内に定義されたアイテムにアクセスできます。
> 単にそれらの名前を使用するだけです。

```rust
struct Ticket {
    // [...]
}

// No need to qualify `Ticket` in any way here
// because we're in the same module
// 同じモジュール内にいるため、ここではなにも`Ticket`を修飾する必要はありません。
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

That's not the case if you want to access an entity from a different module.\
You have to use a **path** pointing to the entity you want to access.

> 異なるモジュールのエンティティにアクセスしたい場合は別です。
> アクセスしたいエンティティを示す**パス**を使用しなければなりません。

You can compose the path in various ways:

- starting from the root of the current crate, e.g. `crate::module_1::module_2::MyStruct`
- starting from the parent module, e.g. `super::my_function`
- starting from the current module, e.g. `sub_module_1::MyStruct`

> さまざまな方法でパスを構成できます。
>
> - 例えば、`crate::module_1::module_2::MyStruct`のように、現在のクレートのルートから開始
> - 例えば、`super::my_function`のうように、親モジュールから開始
> - 例えば、`sub_module_1::MyStruct`のように、現在のモジュールから開始

Having to write the full path every time you want to refer to a type can be cumbersome.
To make your life easier, you can introduce a `use` statement to bring the entity into scope.

> 型を参照したい場合はいつでも、完全なパスを記述しなければならないことは、面倒になる可能性があります。
> 人生を楽にするために、スコープ内にエンティティを持ち込む`use`文を導入できます。

```rust
// Bring `MyStruct` into scope
use crate::module_1::module_2::MyStruct;

// Now you can refer to `MyStruct` directly
fn a_function(s: MyStruct) {
     // [...]
}
```

### Star imports（スターインポート）

You can also import all the items from a module with a single `use` statement.

> また、単一の`use`文を使用して、モジュールからすべてのアイテムをインポートできます。

```rust
use crate::module_1::module_2::*;
```

This is known as a **star import**.\
It is generally discouraged because it can pollute the current namespace, making it hard to understand
where each name comes from and potentially introducing name conflicts.\
Nonetheless, it can be useful in some cases, like when writing unit tests. You might have noticed
that most of our test modules start with a `use super::*;` statement to bring all the items from the parent module
(the one being tested) into scope.

> これは**スターインポート**として知られています。
> それは、それぞれの名前がどこから来たのか理解することを難しくして、名前の衝突を招く可能性があり、現在の名前空間を汚染する可能性があるため、一般的には推奨されません。
> それにも関わらず、ユニットテストを記述しているときなど、スターインポートはいくつかの場面で便利です。
> ほとんどのテストモジュールが、親モジュールからテストされるすべてのアイテムをスコープ内に持ち込むために、`user super::*;`文で開始していることに気付いたかもしれません。
