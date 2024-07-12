// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
// 整数のベクターを与えて、それを二分割して、別々のスレッドでそれぞれの半分の合計を計算してください。
// ヒープを確保しないでください。メモリーをリークしないでください。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let middle = v.len() / 2;
    thread::scope(|scope| {
        let handle1 = scope.spawn(|| v[..middle].iter().sum::<i32>());
        let handle2 = scope.spawn(|| v[middle..].iter().sum::<i32>());

        handle1.join().unwrap() + handle2.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
