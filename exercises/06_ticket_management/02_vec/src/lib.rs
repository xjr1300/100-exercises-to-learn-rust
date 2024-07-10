// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
// 数値`n`を与えて、`n+1`個の数のフィボナッチ数列を返してください。
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// フィボナッチ数列は次の通り定義されます。
//
// - 数列の最初の数は0です。
// - 数列の2番目の数は1です。
// - すべての部分数列の数は、前の2つの数の和です。
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// よって数列は次のようになります。0, 1, 1, 2, 3, 5, 8, 13, 21の用に続きます。
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
// `fibonacci(0)`が`0`を返し、`fibonacci(1)`が`1`を返し、`fibonacci(2)`が`1`を返し、
// そしてそれに続くことを期待します。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    // `fibonacci`関数を実装してください。
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    // ヒント: それらを何度も再計算しないように、すでに計算された結果をメモ化するために`Vec`を
    // 使用してください。
    let n = n as usize;
    let mut values: Vec<u32> = Vec::new();
    for n in 0..=n {
        if n == 0 {
            values.push(0);
        } else if n == 1 {
            values.push(1);
        } else {
            let value = values[n - 1] + values[n - 2];
            values.push(value);
        }
    }

    values[n]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
