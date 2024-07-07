# The `Drop` trait（Dropトレイト）

When we introduced [destructors](../03_ticket_v1/11_destructor.md),
we mentioned that the `drop` function:

1. reclaims the memory occupied by the type (i.e. `std::mem::size_of` bytes)
2. cleans up any additional resources that the value might be managing (e.g. the heap buffer of a `String`)

> デストラクターを導入したとき、`drop`関数について言及しました。
>
> 1. 例えば`std::mem::size_of`バイトで、その型によって専有されるメモリを回収します。
> 2. 例えば`String`のヒープバッファーなど、その値が管理しているかもしれない任意の追加リソースをクリーンアップします。

Step 2. is where the `Drop` trait comes in.

> ステップ2は、`Drop`トレイトが登場する場面です。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

The `Drop` trait is a mechanism for you to define _additional_ cleanup logic for your types,
beyond what the compiler does for you automatically.\
Whatever you put in the `drop` method will be executed when the value goes out of scope.

> `Drop`トレイトは、コンパイラーが自動的に行うことを超えて、型の _追加の_ クリーンアップロジックを定義するためのメカニズムです。
> `drop`メソッド内に入れたものは何でも、値がスコープ外になったときに実行されます。

## `Drop` and `Copy`（DropとCopy）

When talking about the `Copy` trait, we said that a type can't implement `Copy` if it
manages additional resources beyond the `std::mem::size_of` bytes that it occupies in memory.

> `Copy`トレイトについて話しているとき、型がメモリ内に専有する`std::mem::size_of`バイトを超える追加リソースを管理している場合、その型は`Copy`を実装できないと言いました。

You might wonder: how does the compiler know if a type manages additional resources?
That's right: `Drop` trait implementations!\
If your type has an explicit `Drop` implementation, the compiler will assume
that your type has additional resources attached to it and won't allow you to implement `Copy`.

> 困惑しているかもしれません。コンパイラーは、型が追加リソースを管理していることをどのように知るのでしょうか？
> そのとおりです。`Drop`トレイトをの実装です！
> 型が明示的に`Drop`の実装がある場合、コンパイラーは型が型に付属する追加リソースを持っていると仮定して、`Copy`を実装することを許可しません。

```rust
// This is a unit struct, i.e. a struct with no fields.
// これはユニット構造体で、つまりフィールドのない構造体です。
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // We don't need to do anything here,
       // it's enough to have an "empty" Drop implementation
       // ここで何もする必要はありません。
       // 「空の」Drop実装があれば十分です。
    }
}
```

The compiler will complain with this error message:

> コンパイラは、次のメッセージで文句を言うでしょう。

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```
