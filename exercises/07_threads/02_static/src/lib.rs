// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
// 静的な整数のスライスを与えて、スライスを二等分して、別々のスレッドで半分をそれぞれ合計してください。
// 追加メモリを割当しないでください！
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let middle = slice.len() / 2;
    let (slice1, slice2) = slice.split_at(middle);

    let handle1 = thread::spawn(|| slice1.iter().sum::<i32>());
    let handle2 = thread::spawn(|| slice2.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
