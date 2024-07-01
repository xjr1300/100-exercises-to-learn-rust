// TODO: fix the function signature below to make the tests pass.
//  Make sure to read the compiler error message—the Rust compiler is your pair programming
//  partner in this course and it'll often guide you in the right direction!
// TODO: テストにパスするように下の関数のシグネチャーを修正してください。
// コンパイラーのエラーメッセージを読んでください。ラストコンパイラーはこのコースのプログラミングパートナーとなるペアで、
// 通常、正しい方向へ導いてくれます。
//
// The input parameters should have the same type of the return type.
// 入力パラメーターは、戻り値の方と同じ型を保つ必要があります。
fn compute(a: u32, b: u32) -> u32 {
    // Don't touch the function body.
    // 関数本体を触らないでください。
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}
