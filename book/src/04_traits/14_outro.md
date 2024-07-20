# Wrapping up（仕上げ）

We've covered quite a few different traits in this chapter—and we've only scratched the surface!
It may feel like you have a lot to remember, but don't worry: you'll bump into these traits
so often when writing Rust code that they'll soon become second nature.

> この章において、かなり多くのトレイトを説明しました。そしてまだ表面を引っ掻いただけです！
> 覚えておくべき多くのことがあるように感じるかもしれませんが、心配しないでください。
> Rustでコードを記述するとき、頻繁にこれらのトレイトに遭遇するため、すぐにそれが2つ目の習慣になるでしょう。

## Closing thoughts（最後に言っておきたいこと）

Traits are powerful, but don't overuse them.\
A few guidelines to keep in mind:

- Don't make a function generic if it is always invoked with a single type. It introduces indirection in your
  codebase, making it harder to understand and maintain.
- Don't create a trait if you only have one implementation. It's a sign that the trait is not needed.
- Implement standard traits for your types (`Debug`, `PartialEq`, etc.) whenever it makes sense.
  It will make your types more idiomatic and easier to work with, unlocking a lot of functionality provided
  by the standard library and ecosystem crates.
- Implement traits from third-party crates if you need the functionality they unlock within their ecosystem.
- Beware of making code generic solely to use mocks in your tests. The maintainability cost of this approach
  can be high, and it's often better to use a different testing strategy. Check out the
  [testing masterclass](https://github.com/mainmatter/rust-advanced-testing-workshop)
  for details on high-fidelity testing.

> トレイトは強力ですが、それらを乱用しないでください。
> 次のいくつかのガイドラインを心に留めておいてください。
>
> - 常に単一の型で関数が呼び出される場合はジェネリックにしないでください。
>   それはコードベースに間接性を導入して、理解することと維持することを難しくします。
> - 単一の実装しかない場合、トレイトを作成しないでください。それはトレイトが必要とされない合図です。
> - `Debug`、`PartialEq`など標準トレイトを型に実装するときはいつでも、意味があるようにしてください。
>   それは、型をより慣用的に、一緒に機能することを容易にして、標準ライブラリやエコシステムクレートによって提供される多くの機能を解放します。
> - 必要に応じてサードパーティクレイトのトレイトを実装して、それらのエコシステムが持つ機能を解放してください。
> - テストでモックを使用するためだけにコードをジェネリックにすることに注意してください。
>   このアプローチの保守コストは高くなり、時々異なるテスト戦略を使用する方が良いことがあります。
>   高い忠実度をもつテストについての詳細は、テストマスタークラスを確認してください。

## Testing your knowledge（知識のテスト）

Before moving on, let's go through one last exercise to consolidate what we've learned.
You'll have minimal guidance this time—just the exercise description and the tests to guide you.

> 次に進む前に、学んだことを固めるために最後の演習を1つ行いましょう。
> 今回は、最小限の案内があります。それは、単なる演習の説明と、テストの案内です。
