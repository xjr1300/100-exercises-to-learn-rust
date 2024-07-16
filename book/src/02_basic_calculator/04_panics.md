# Panics（パニック）

Let's go back to the `speed` function you wrote for the ["Variables" section](02_variables.md).
It probably looked something like this:

> 「変数」節で記述した`speed`関数に戻りましょう。
> それはおそらく次のように見えたでしょう。

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

If you have a keen eye, you might have spotted one issue[^one]: what happens if `time_elapsed` is zero?

> もし注意深い目を持っている場合、1つの問題点があることに気づいたかもしれません。`time_elapses`がゼロの場合、何が起こるでしょうか？

You can try it
out [on the Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac)!\
The program will exit with the following error message:

> `Rust playground`で試すことができます！
> プログラムは、次のエラーメッセージで終了します。

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

> ```text
> `main`スレッドは、src/main.rs:3:5でパニックしました：
> ゼロで割ろうと試みています
> ```

This is known as a **panic**.\
A panic is Rust's way to signal that something went so wrong that
the program can't continue executing, it's an **unrecoverable error**[^catching]. Division by zero classifies as such an
error.

> これは**パニック**として知られています。
> パニックは、プログラムが実行を継続できない何か悪い状態になったことを示すRustの方法です。
> それは**回復不能なエラー**です。
> ゼロ割りはそのようなエラーに分類されます。

## The panic! macro（panic!マクロ）

You can intentionally trigger a panic by calling the `panic!` macro[^macro]:

> `panic!`マクロを呼び出すことで、意図的にパニックを起こせます。

```rust
fn main() {
    panic!("This is a panic!");
    // The line below will never be executed
    // 下の行は決して実行されません
    let x = 1 + 2;
}
```

There are other mechanisms to work with recoverable errors in Rust, which [we'll cover later](../05_ticket_v2/06_fallibility.md).
For the time being we'll stick with panics as a brutal but simple stopgap solution.

> Rustには回復可能なエラーと一緒に機能する他のメカニズムがあり、それは後で説明します。
> 当分は、野蛮ですが単純な一時しのぎの解決策としてパニックに固執します。

## Further reading（参考資料）

- [The panic! macro documentation](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: There's another issue with `speed` that we'll address soon enough. Can you spot it?
`speed`には十分すぐに対処できる別の問題があります。それを見つけることはできますか？

[^catching]: You can try to catch a panic, but it should be a last resort attempt reserved for very specific
circumstances.
パニックをキャッチすることを試みることができますが、それはとても特定な状況に予約された最後の手段であるべきです。

[^macro]: If it's followed by a `!`, it's a macro invocation. Think of macros as spicy functions for now. We'll
cover them in more detail later in the course.
それに`!`が続く場合、それはマクロ呼び出しです。
現段階では、スパイシーな関数としてマクロを考えてください。
コースの後半で詳細にそれらを説明します。
