// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
// 非負の整数`n`を受け取り、`n`の階乗である`n!`を返す`factorial`と名付けた関数を定義してください。
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
// `n`の階乗は、`n`までの正の整数の積として定義されます。
// 例えば、`5!`（「5の階乗」と読みます）は、`5 * 4 * 3 * 2 * 1`で、それは`120`です。
// `0!`は`1`と定義されています。
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
// `factorial(0)`が`1`を返し、`factorial(1)`が`1`を返し、`factorial(2)`が`2`を返す、といったことを期待します。
//
// Use only what you learned! No loops yet, so you'll have to use recursion!
// 学んだことだけを歯悠桜してください！ループはまだなので、再帰を使用する必要があります。

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

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
