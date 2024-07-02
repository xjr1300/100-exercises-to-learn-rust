// 👇 The lines below, starting with `///`, are called **documentation comments**.
//    They attach documentation to the item that follows them. In this case, the `speed` function.
//    If you run `cargo doc --open` from this exercise's directory, Rust will generate
//    HTML documentation from these comments and open it in your browser.
//    `///`で始まっている下の行は、**ドキュメンテーションコメント**です。
//    それらは、後続する項目にドキュメントを添付します。個の場合、`speed`関数です。
//    この演習ディレクトリから`cargo doc --open`を実行した場合、RustはこれらのコメントからHTMLドキュメントを生成して、
//    ブラウザ内でそれを表示します。

/// Given the start and end points of a journey, and the time it took to complete it,
/// calculate the average speed.
/// 旅行の開始と終了点と、その完了までにかかった時間を与えると、平均速度を計算します。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    //  Do you need to annotate the type of `distance`? Why or why not?
    // TODO: テストにパスするように正しい値で`distance`と名付けられた変数を定義してください。
    // `distance`の型を注釈する必要はありますか？なぜでしょうか、またはなぜいらないのでしょうか？
    let distance = end - start;

    // Don't change the line below
    // 下の行は変更しないでください。
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
