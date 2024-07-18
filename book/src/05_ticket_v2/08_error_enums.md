# Error enums（Error列挙型）

Your solution to the previous exercise may have felt awkward: matching on strings is not ideal!\
A colleague might rework the error messages returned by `Ticket::new` (e.g. to improve readability) and,
all of a sudden, your calling code would break.

> 前の演習の解答は、洗練されていないかもしれません。文字列にマッチングすることは理想的ではありません！
> 同僚は、例えば可読性を改善するために`Ticket::new`によって返されたメッセージを再構築するかもしれず、突然コードの呼び出しは壊れます。

You already know the machinery required to fix this: enums!

> 列挙型でこれを修正するために要求されるメカニズムをすでに知っています！

## Reacting to errors（エラーに反応する）

When you want to allow the caller to behave differently based on the specific error that occurred, you can
use an enum to represent the different error cases:

> 発生した特定のエラーに基づいて、呼び出し側に異なる振る舞いをさせたいとき、異なるエラーケースを表現する列挙型を使用できます。

```rust
// An error enum to represent the different error cases
// that may occur when parsing a `u32` from a string.
// 文字列から`u32`にパースするときに発生するかもしれない異なるエラーケースを表現するエラー列挙型です。
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Using an error enum, you're encoding the different error cases in the type system—they become part of the
signature of the fallible function.\
This simplifies error handling for the caller, as they can use a `match` expression to react to the different
error cases:

> エラー列挙型を使用することは、型システムにさまざまなエラーケースを符号化することです。
> それらは、失敗する可能性のある関数のシグネチャーの一部になります。
> 異なるエラーケースに対して反応するために`match`式を使用できるため、これは呼び出し側のエラー処理を簡素化します。

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```
