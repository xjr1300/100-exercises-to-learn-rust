// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
// `self`を`n`場する`power`メソッドを持つ`Power`トレイトを定義してください。
// そのトレイトの定義とその実装は、テストをコンパイルしてパスするようにしなければなりません。
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.
// 推奨事項： 一度にすべてのクレートを処理するために、ジェネリックな実装を一時的にするかもしれません。
// しかし、これはかなり複雑で、例えば`num-traits`などの追加クレートを要求します。
// それでも、それは高次元なジェネリックの複雑性を避けるために、代わりに単純なマクロを使用することが
// 好ましいかもしれません。
// それについてより学ぶことに興味がある場合、"Little book of Rust macros"を確認してください。
// しかし、そのようにする必要もありません。
// 3つの分離した実装を手動で記述してもまったく大丈夫です。
// 興味がある場合にのみ。、さらに進んでください。

trait Power<T> {
    type Output;

    fn power(&self, n: T) -> Self::Output;
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, n: &u32) -> Self::Output {
        self.pow(*n)
    }
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
