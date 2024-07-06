pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    // コンパイル時エラーの結果になる、`std::mem::size_of`を使用してstrのサイズを取得することを試みてください。
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // 次の行をコメントアウトして、次の演習に進んでください。
    // std::mem::size_of::<str>();
}

/*
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> exercises/04_traits/08_sized/src/lib.rs:8:25
    |
8   |     std::mem::size_of::<str>();
    |                         ^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `std::mem::size_of`
   --> /Users/xjr1300/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/mod.rs:312:22
    |
312 | pub const fn size_of<T>() -> usize {
    |                      ^ required by the implicit `Sized` requirement on this type parameter in `size_of`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `sized` (lib) due to 1 previous error
error: could not compile `sized` (lib test) due to 1 previous error
 */
