// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.
// TODO: このセクションで学んだことに基づいて、`todo!()`を変換後の正確な値に置き換えてください。
#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        // コンパイラーは値255をi8に収めることができないことを理解するほど十分に賢いため、コンパイラーはエラーを発します。
        // この（悪い）変換を可能にするために、意図的にこのガードレールを無効にします。
        // コンパイラーがこれを検出できるのは、値がリテラルであるためです。
        // 変数を使用した場合、コンパイラーはコンパイル時にこれを検出できません。
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // `i8` value that is equivalent to `255` when converted from `u8`.
        // 上記と同じ式を正確に使用することでこれを解決できますが、それは演習目的を無効にします。
        // 代わりに、`u8`から変換されたときに、`255`と等しい純正の`i8`値を使用します。
        let y: i8 = -1;

        /*
        255のビット表現は次のとおり。
        1 1 1 1 1 1 1 1
        これを2の補数で表現された値と考えると、-1である。
        */

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
