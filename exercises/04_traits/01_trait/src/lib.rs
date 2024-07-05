// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
// `self`が偶数の場合に`true`を、そうでない場合に`false`を返す`is_even`メソッドを持つ`IsEven`と名付けたトレイトを
// 定義してください。
//
// Then implement the trait for `u32` and `i32`.
// そして、`u32`と`i32`にそのトレイトを実装してください。

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
