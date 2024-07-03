// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.
//
// 次のフィールドを持つ`Order`と名付けた構造体を定義してください。
// - 符号なし整数の`price`
// - 符号なし整数の`quantity`
//
// それは、数量が0より大きい場合に`true`、そうでない場合に`false`を返す、`is_available`と名付けられたメソッドも持ちます。

struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
