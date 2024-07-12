// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.
// `spawn`と`join`を使用して、`sum`関数のマルチスレッドバーションを実装してください。
// 与えられた整数のベクターを二等分して、別々のスレッドでそれぞれの半分を合計します。

//use core::num;
// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
// 注意事項: *どのように*関数が実装されるかテストできません。
// それが正しい結果を生成することのみを検証できます。
// 単に`v.iter().sum()`を返すことによって、このテストにパス_できます_が、
// 演習の目的を台無しにします。
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
// ヒント: 生み出したスレッドが直接ベクターのスライスを_借用_することはできません。
// オリジナルなベクターのそれぞれの半分のために新しいベクターを割当てる必要があります。
// 次の演習でこれが必要な理由を確認します。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let number_of_half = v.len() / 2;
    // イテレーターは要素の参照を返すため、値を所有するために`.cloned()`を使用する。
    let former = v[0..number_of_half].iter().cloned().collect::<Vec<i32>>();
    let latter = v[number_of_half..].iter().cloned().collect::<Vec<i32>>();

    let former_handle = thread::spawn(move || former.iter().sum::<i32>());
    let latter_handle = thread::spawn(move || latter.iter().sum::<i32>());

    former_handle.join().unwrap() + latter_handle.join().unwrap()
}

/*
pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();

    let handle1 = thread::spawn(move || v1.iter().sum::<i32>());
    let handle2 = thread::spawn(move || v2.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}
 */

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
