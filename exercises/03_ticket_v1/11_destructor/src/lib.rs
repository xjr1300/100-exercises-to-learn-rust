// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
// デストラクター用に適切な演習を記述するために、なんらかの機械を必要とします。
// トレイトと内部可変性を説明した後のチャプターで、再度概念を取り上げます。
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
