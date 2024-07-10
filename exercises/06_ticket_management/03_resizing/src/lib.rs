#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached（最大容量に達した）
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize（容量を超えたため、リサイズが必要）

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        // 新しい容量がいくつか推測できますか？
        // 標準ライブラリは、ベクターのリサイズに使用するアルゴリズムについて保証してないため、
        // これは将来変更されるかもしれないことに注意してください。
        assert_eq!(v.capacity(), 4);
    }
}
