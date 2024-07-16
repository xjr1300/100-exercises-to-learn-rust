# Overflow（オーバーフロー）

The factorial of a number grows quite fast.\
For example, the factorial of 20 is 2,432,902,008,176,640,000. That's already bigger than the maximum value for a
32-bit integer, 2,147,483,647.

> 階乗の数はとても早く増加します。
> 例えば、20の階乗は2,432,902,008,176,640,000です。これはすでに32ビット整数の最大値である2,147,483,647よりも大きいです。

When the result of an arithmetic operation is bigger than the maximum value for a given integer type,
we are talking about **an integer overflow**.

> 算術操作の結果が与えられた整数型の最大値よりも大きくなるとき、それは**整数オーバーフロー**と呼ばれます。

Integer overflows are an issue because they violate the contract for arithmetic operations.\
The result of an arithmetic operation between two integers of a given type should be another integer of the same type.
But the _mathematically correct result_ doesn't fit into that integer type!

> 整数オーバーフローは、それらが算術操作の契約に違反するため問題です。
> 型が与えられた2つの整数の算術演算の結果は、同じ型の別な値になるべきです。
> しかし、*数学的に正確な結果*は、その整数型に収まりません！

> If the result is smaller than the minimum value for a given integer type, we refer to the event as **an integer
> underflow**.\
> For brevity, we'll only talk about integer overflows for the rest of this section, but keep in mind that
> everything we say applies to integer underflows as well.
>
> The `speed` function you wrote in the ["Variables" section](02_variables.md) underflowed for some input
> combinations.
> E.g. if `end` is smaller than `start`, `end - start` will underflow the `u32` type since the result is supposed
> to be negative but `u32` can't represent negative numbers.

> 結果が与えられた整数型の最小値よりも小さい場合、そのイベントを**整数アンダーフロー**と呼びます。
> 簡潔にするために、この節の残りは整数オーバーフローについて話しますが、ここで話すすべてのことは整数アンダーフローにも適用することを心に留めておいてください。
>
> 「変数」節で記述した`speed`関数は、任意の入力の組み合わせでアンダーフローします。
> 例えば、`end`が`start`よりも小さい場合、`end - start`は、その結果は負が想定されますが、`u32`は負数を表現できないため、`u32`型でアンダーフローします。

## No automatic promotion（自動昇格はありません）

One possible approach would be automatically promote the result to a bigger integer type.
E.g. if you're summing two `u8` integers and the result is 256 (`u8::MAX + 1`), Rust could choose to interpret the
result as `u16`, the next integer type that's big enough to hold 256.

But, as we've discussed before, Rust is quite picky about type conversions. Automatic integer promotion
is not Rust's solution to the integer overflow problem.

> 1つの可能なアプローチは、結果をより大きな整数型に自動的に昇格させることです。
> 例えば、2つの`u8`整数を合計して、結果が256（`u8::MAX + 1`）になる場合、Rustは`u16`として結果を解釈することを選択するかもしれません。
> 次の整数型は、256を保持するほど十分大きいです。
>
> しかし、前に議論したように、Rustは型変換についてとてもうるさいです。
> 自動整数昇格は、整数オーバーフロー問題に対するRustの解決策ではありません。

## Alternatives（代替手段）

Since we ruled out automatic promotion, what can we do when an integer overflow occurs?\
It boils down to two different approaches:

- Reject the operation
- Come up with a "sensible" result that fits into the expected integer type

> 自動昇格を除外したため、整数オーバーフローが発生したとき何ができるでしょうか？
> 2つの異なるアプローチにまとめられます。
>
> - 操作を拒否する
> - 期待される整数型にフィットする「良識ある」結果を考える

### Reject the operation（操作を拒否する）

This is the most conservative approach: we stop the program when an integer overflow occurs.\
That's done via a panic, the mechanism we've already seen in the ["Panics" section](04_panics.md).

> これは最も保守的なアプローチです。
> 整数オーバーフローが発生したときプログラムを停止します。
> これはパニックを介して行われ、そのメカニズムは、「パニック」節ですでに確認しました。

### Come up with a "sensible" result（【良識ある」結果を考える）

When the result of an arithmetic operation is bigger than the maximum value for a given integer type, you can
choose to **wrap around**.\
If you think of all the possible values for a given integer type as a circle, wrapping around means that when you
reach the maximum value, you start again from the minimum value.

> 算術操作の結果が与えられた整数型の最大値よりも大きくなったとき、**包み込む**ことを選択できます。
> 円として与えられた整数型ですべての可能性のある値を考えると、包み込みは最大値に到達したとき、最小値から再び開始することを意味します。

For example, if you do a **wrapping addition** between 1 and 255 (=`u8::MAX`), the result is 0 (=`u8::MIN`).
If you're working with signed integers, the same principle applies. E.g. adding 1 to 127 (=`i8::MAX`) with wrapping
will give you -128 (=`i8::MIN`).

> 例えば、1と255 (=`u8::MAX`)の**包み込み加算**をすると、結果は0（=`u8::MIN`）になります。
> 符号付き整数で作業している場合、同じ原則が適用されます。
> 例えば、包み込みを使用した1と127（=`i8::MAX`）の加算は、-128（=`i8::MIN`）を与えます。

## `overflow-checks`（オーバーフロー検査）

Rust lets you, the developer, choose which approach to use when an integer overflow occurs.
The behaviour is controlled by the `overflow-checks` profile setting.

> Rustは、整数オーバーフローが発生したときに使用するアプローチを開発者に選択できるようにします。
> その振る舞いは、`overflow-checks`プロファイル設定によって制御されます。

If `overflow-checks` is set to `true`, Rust will **panic at runtime** when an integer operation overflows.
If `overflow-checks` is set to `false`, Rust will **wrap around** when an integer operation overflows.

> `overflow-checks`が`true`に設定されている場合、Rustは整数オーバーフローが発生したとき**ランタイムでパニック**します。
> `overflow-checks`が`false`に設定されている場合、Rustは整数オーバーフローが発生したとき**包み込み**します。

You may be wondering—what is a profile setting? Let's get into that!

> 困惑しているかもしれません。プロファイル設定とは何でしょうか？
> それについて説明しましょう！

## Profiles（プロファイル）

A [**profile**](https://doc.rust-lang.org/cargo/reference/profiles.html) is a set of configuration options that can be
used to customize the way Rust code is compiled.

> **プロファイル**は、Rustコードがコンパイルされる方法をカスタマイズするために使用できる構成オプションのセットです。

Cargo provides two built-in profiles: `dev` and `release`.\
The `dev` profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local
development,
therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.\
The `release` profile, instead, is optimized for runtime performance but incurs longer compilation times. You need
to explicitly request via the `--release` flag—e.g. `cargo build --release` or `cargo run --release`.

> Cargoは、`dev`と`release`の2つの組み込みプロファイルを提供します。
> `dev`プロファイルは、`cargo build`、`cargo run`または`cargo test`を実行するたびに使用されます。
> それはローカル開発を目的としているため、それは短いコンパイル時間とより良いデバッグ体験を得るために、ランタイムの性能を犠牲にしています。
> 代わりに、`release`プロファイルはランタイム性能のために最適化されますが、より長いコンパイル時間が必要です。
> 明示的に`--release`フラグを要求する必要があります。
> 例えば、`cargo build --release`または`cargo run --release`です。

> "Have you built your project in release mode?" is almost a meme in the Rust community.\
> It refers to developers who are not familiar with Rust and complain about its performance on
> social media (e.g. Reddit, Twitter, etc.) before realizing they haven't built their project in
> release mode.

> 「リリースモードでプロジェクトをビルドしましたか？」は、Rustコミュニティでほぼミーム（文化の中で人から人に広がっていく行動やアイデアのこと）です。
> それは、Rustに慣れていない開発者が、リリースモードでプロジェクトをビルドしていないことに気づく前に、Redditやツイッターなどのソーシャルメディアでその性能について不満を言うことを示します。

You can also define custom profiles or customize the built-in ones.

> また、カスタムプロファイルを定義したり、組み込みのプロファイルをカスタマイズすることもできます。

### `overflow-check`（オーバーフロー確認）

By default, `overflow-checks` is set to:

- `true` for the `dev` profile
- `false` for the `release` profile

> デフォルトで、`overflow-checks`は次の通り設定されています。
>
> `true`は`dev`プロファイルに対して
> `false`は`release`プロファイルに対して

This is in line with the goals of the two profiles.\
`dev` is aimed at local development, so it panics in order to highlight potential issues as early as possible.\
`release`, instead, is tuned for runtime performance: checking for overflows would slow down the program, so it
prefers to wrap around.

> これは、2つのプロファイルの目標と一致しています。
> `dev`はローカル開発を目的としているため、それは可能な限り早期に問題の可能性を強調するためにパニックします。
> 代わりに、`release`は、ランタイム性能を向上させるために調整されており、オーバーフローの確認はプログラムを遅くするため、それは包み込むことを選択しています。

At the same time, having different behaviours for the two profiles can lead to subtle bugs.\
Our recommendation is to enable `overflow-checks` for both profiles: it's better to crash than to silently produce
incorrect results. The runtime performance hit is negligible in most cases; if you're working on a performance-critical
application, you can run benchmarks to decide if it's something you can afford.

> 同時に、2つのプロファイルで異なる振る舞いを持つことで、些細なバグを招く可能性があります。
> 推奨は、両方のプロファイルで`overflow-checks`を有効にすることです。
> それは、静かに誤った結果を生産するよりも、クラッシュしたほうが良いからです。
> ランタイム性能の影響は、ほとんどの場合で無視できます。
> パフォーマンスが重要なアプリケーションで作業している場合、ベンチマークを実行して、それが許容できるか決定できます。

## Further reading（参考資料）

- Check out ["Myths and legends about integer overflow in Rust"](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/)
  for an in-depth discussion about integer overflow in Rust.
