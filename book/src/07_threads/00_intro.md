# Intro（導入）

One of Rust's big promises is _fearless concurrency_: making it easier to write safe, concurrent programs.
We haven't seen much of that yet. All the work we've done so far has been single-threaded.
Time to change that!

> Rustの大きな約束は、_恐れを知らない同時並行_です。それは安全に並同時並行プログラムを記述できるようにします。
> これまではあまり確認しませんでした。これまでに行ってきたすべての作業は単一スレッドでした。
> それを変える時がきました！

In this chapter we'll make our ticket store multithreaded.\
We'll have the opportunity to touch most of Rust's core concurrency features, including:

- Threads, using the `std::thread` module
- Message passing, using channels
- Shared state, using `Arc`, `Mutex` and `RwLock`
- `Send` and `Sync`, the traits that encode Rust's concurrency guarantees

> この章において、マルチスレッドなチケットストアを作成します。
> 次のようなRustの主要な同時並行機能の殆どに触れる機会を持ちます。
>
> - `std::thread`モジュールを使用したスレッド
> - チャネルを使用したメッセージ送信
> - `Arc`、`Mutex`そして`RwLock`を使用した共有された状態
> - Rustの同時並行性の保証を符号化する`Send`と`Sync`トレイト

We'll also discuss various design patterns for multithreaded systems and some of their trade-offs.

> また、マルチスレッドシステムのためのさまざまなデザインパターンと、それらのいくつかの二律背反についても議論します。
