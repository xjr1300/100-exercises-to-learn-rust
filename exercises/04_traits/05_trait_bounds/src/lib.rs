// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
// コンパイルが成功するように、`min`に必要なトレイト制約を追加してください。
// 必要の応じてより情報を得るために`std::cmp`モジュールのドキュメントを参照してください。
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).
// 注意事項: コンパイラーを満足させるさまざまなトレイト制約がありますが、それらは異なる*意味*を伴います。
// 例えば`BTreeMap`のような順番付されたコレクションについて話すとき、コースの後の方でそれらの違いを説明するつもりです。

/// Return the minimum of two values.
/// 2つの値の最小値を返します。
pub fn min<T>(left: T, right: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if left <= right {
        left
    } else {
        right
    }
}
