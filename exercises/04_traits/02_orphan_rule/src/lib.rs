// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.
// これは、孤立ルール違反の例です。
// `std`の`u32`の外部の型に対して、`std`の`PartialEq`の外部のトレイトを実装しています。
// どのように見えるか慣れるために、コンパイルエラーを確認してください。
// そして、下のコードを削除して、次の演習に進んでください。

/*
impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
 */

/*
error[E0117]: only traits defined in the current crate can be implemented for primitive types
  --> exercises/04_traits/02_orphan_rule/src/lib.rs:10:1
   |
10 | impl PartialEq for u32 {
   | ^^^^^---------^^^^^---
   | |    |             |
   | |    |             `u32` is not defined in the current crate
   | |    `u32` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
error: could not compile `orphan` (lib) due to 1 previous error
error: could not compile `orphan` (lib test) due to 1 previous error
 */
