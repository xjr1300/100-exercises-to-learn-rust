// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.
// 「ドロップ爆弾】と呼ばれるモノを実装してください。その型はドロップされたとき、それに特定の操作を実装せずにパニックします。
// 下のテストで期待されたAPIを確認できます。

struct DropBomb {
    defused: bool,
}

impl DropBomb {
    fn new() -> Self {
        Self { defused: false }
    }

    fn defuse(&mut self) {
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Boom!!!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
        // ドロップされたとき爆弾はパニックしなければなりません。
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
        // 爆弾が解除されたため、ドロップされたとき爆弾はパニックしてはなりません。
    }
}
