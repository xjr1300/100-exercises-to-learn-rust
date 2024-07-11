// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.
// `u32`のスライスへの参照を受け取り、スライス内のすべての要素の合計を返す、`sum`と名付けた関数を定義してください。

fn sum(values: &[u32]) -> u32 {
    values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
