# Traits（トレイト）

In the previous chapter we covered the basics of Rust's type and ownership system.\
It's time to dig deeper: we'll explore **traits**, Rust's take on interfaces.

> 前の章で、Rustの型と所有権システムの基本を説明しました。
> より深く掘り下げる時間です。Rustのインターフェイスに対する考え方である**トレイト**を探求します。

Once you learn about traits, you'll start seeing their fingerprints all over the place.\
In fact, you've already seen traits in action throughout the previous chapter, e.g. `.into()` invocations as well
as operators like `==` and `+`.

> 一旦、トレイトについて学ぶと、その指紋があらゆる場所で見るようになります。
> 事実、前の章を通じて実際にトレイトを見ました。例えば、`.into()`呼び出しと同様に`==`や`+`のような演算子です。

On top of traits as a concept, we'll also cover some of the key traits that are defined in Rust's standard library:

- Operator traits (e.g. `Add`, `Sub`, `PartialEq`, etc.)
- `From` and `Into`, for infallible conversions
- `Clone` and `Copy`, for copying values
- `Deref` and deref coercion
- `Sized`, to mark types with a known size
- `Drop`, for custom cleanup logic

> 概念としてのトレイトを踏まえた上で、Rust標準ライブラリ内に定義されたいくつか主要なトレイトを説明します。

Since we'll be talking about conversions, we'll seize the opportunity to plug some of the "knowledge gaps"
from the previous chapter—e.g. what is `"A title"`, exactly? Time to learn more about slices too!

> 変換について話す予定であるため、前の章からのいくつかの「知識の乖離」を埋めるために機会を掴みます。
> 例えば、正確に「`"A title"`」とは何でしょうか？またスライスについてより学ぶ時です！
