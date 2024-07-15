//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
//! `example`関数の文を**順序を変更する**ことによってコンパイルできるようにコードを修正してください。
//! `spawner`関数または`example`内の行を変更することはできません。
//! 必要に応じて、`{}`ブロック内に既存の文をラップしてできます。
use std::rc::Rc;
use tokio::task::yield_now;

#[allow(dead_code)]
fn spawner() {
    tokio::spawn(example());
}

#[allow(dead_code)]
async fn example() {
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}
