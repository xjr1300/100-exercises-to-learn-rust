# Async Rust（非同期Rust）

Threads are not the only way to write concurrent programs in Rust.\
In this chapter we'll explore another approach: **asynchronous programming**.

> Rustにおいて、スレッドは同時並行プログラムを記述する唯一の方法ではありません。
> このチャプターにおいて、**非同期プログラミング**という他の手法を探求します。

In particular, you'll get an introduction to:

- The `async`/`.await` keywords, to write asynchronous code effortlessly
- The `Future` trait, to represent computations that may not be complete yet
- `tokio`, the most popular runtime for running asynchronous code
- The cooperative nature of Rust asynchronous model, and how this affects your code

> 特に、以下について紹介する予定です。
>
> - 苦労せずに非同期コードを記述するための`async`/`.await`キーワード
> - まだ完了していないかもしれない計算を表現する`Future`トレイト
> - 非同期コードを実行する最も人気のあるランタイムである`tokio`
> - Rustの非同期モデルの協調的な性質と、これがどのようにコードに影響を与えるか
