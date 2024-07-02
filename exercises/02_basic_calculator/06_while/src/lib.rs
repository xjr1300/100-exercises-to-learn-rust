// Rewrite the factorial function using a `while` loop.
// `while`ループを使用して階乗関数を再記述してください。
pub fn factorial(mut n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    // `todo!()`マクロは、「後でこれに戻ります」としてコンパイラーが解釈するプレースホルダーで、型エラーを抑制します。
    // ランタイムでそれはパニックします。
    let mut result = 1;
    while 2 <= n {
        result *= n;
        n -= 1;
    }

    result
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
