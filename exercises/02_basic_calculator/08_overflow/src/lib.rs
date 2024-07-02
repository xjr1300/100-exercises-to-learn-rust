// Customize the `dev` profile to wrap around on overflow.
// Check Cargo's documentation to find out the right syntax:
// https://doc.rust-lang.org/cargo/reference/profiles.html
// オーバーフローを包み込むために`dev`プロファイルをカスタマイズしてください。
// 正しい構文を見つけるために、Cargoのドキュメントを確認してください。
//
// For reasons that we'll explain later, the customization needs to be done in the `Cargo.toml`
// at the root of the repository, not in the `Cargo.toml` of the exercise.
// 後で説明する理由で、リポジトリのルートにある`Cargo.toml`の中で行う必要があり、演習の`Cargo.toml`の中ではありません。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        // With the default dev profile, this will panic when you run `cargo test`
        // We want it to wrap around instead
        // 20!は2432902008176640000で、それはu32内に収まるほどとても大きいです。
        // デフォルトのdevプロファイルでは、`cargo test`を実行したときパニックします。
        // 代わりにそれを包み込みます。
        //
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // A large number literal using underscores to improve readability!
        // 可読性を改善するために、アンダースコアを使用して大きな数のリテラルを指定してください。
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
