# Outro

Rust's asynchronous model is quite powerful, but it does introduce additional
complexity. Take time to know your tools: dive deep into `tokio`'s documentation
and get familiar with its primitives to make the most out of it.

> Rustの非同期モデルはとても強力ですが、追加の複雑さを導入します。
> ツールを理解する時間を取ってください。
> `tokio`のドキュメントに深入りして、最大限に利用するためにその構成要素に慣れてください。

Keep in mind, as well, that there is ongoing work at the language and `std` level
to streamline and "complete" Rust's asynchronous story. You may experience some
rough edges in your day-to-day work due to some of these missing pieces.

> 同様に、Rustの非同期ストーリーを合理化して「完了」するための作業が、言語と`std`レベルで作業中であることに留意してください。
> これらの欠落している部分のために、日々の作業でいくつかの荒れたエッジを経験するかもしれません。

A few recommendations for a mostly-pain-free async experience:

- **Pick a runtime and stick to it.**\
  Some primitives (e.g. timers, I/O) are not portable across runtimes. Trying to
  mix runtimes is likely to cause you pain. Trying to write code that's runtime
  agnostic can significantly increase the complexity of your codebase. Avoid it
  if you can.
- **There is no stable `Stream`/`AsyncIterator` interface yet.**\
  An `AsyncIterator` is, conceptually, an iterator that yields new items
  asynchronously. There is ongoing design work, but no consensus (yet).
  If you're using `tokio`, refer to [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/)
  as your go-to interface.
- **Be careful with buffering.**\
  It is often the cause of subtle bugs. Check out
  ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html)
  for more details.
- **There is no equivalent of scoped threads for asynchronous tasks**.\
  Check out ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/)
  for more details.

> 非同期体験をほとんど痛みなくするためのいくつかの推奨事項があります。
>
> - **ランタイムを選択して、それに固執してください**
>   タイマーやI/Oなどのいくつかの構成要素は、ランタイムをまたいて持ち運びできません。
>   ランタイムを混ぜることは、痛みを引き起こす可能性があります。
>   ランタイムを理解しないコードを記述することは、コードベースの複雑さを大幅に増加させる可能性があります。
>   可能であれば、それを避けてください。
> - **まだ安定した`Stream`/`AsyncIterator`インターフェイスはありません。
>   概念的に`AsyncIterator`は、非同期で新しいアイテムを生み出すイテレーターです。
>   設計作業中ですが、いまだ合意が得られていません。
>   `tokio`を使用している場合、インターフェースとして`tokio_stream`を参照してください。
> - **バッファリングに注意してください**
>   それはしばしば微妙なバグの原因になります。詳細は「Barbara battles buffered streams」を確認してください。
> - **非同期タスクにスコープが制限されたスレッドと同等なものはありません**
>   詳細は「The scoped task trilemma」を確認してください。

Don't let these caveats scare you: asynchronous Rust is being used effectively
at _massive_ scale (e.g. AWS, Meta) to power foundational services.\
You will have to master it if you're planning building networked applications
in Rust.

> これらの注意点に怯えないでください。
> 非同期Rustは、基盤となるサービスを強化するために、例えばAWS、Metaなどの_巨大な_スケールで効果的に使用されています。
> Rustでネットワークアプリケーションの構築を計画している場合、それをマスターする必要があります。
