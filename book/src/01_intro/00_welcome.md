# Welcome（ようこそ）

Welcome to **"100 Exercises To Learn Rust"**!

> ようこそ、**"Rustを学ぶための100個の演習**へ!

This course will teach you Rust's core concepts, one exercise at a time.\
You'll learn about Rust's syntax, its type system, its standard library, and its ecosystem.

> このコースは、1回で1つの演習によって、Rustの核となる概念を教えます。
> Rustの構文、その型システム、その標準ライブラリ、そしてそのエコシステムについて学びます。

We don't assume any prior knowledge of Rust, but we assume you know at least
another programming language.
We also don't assume any prior knowledge of systems programming or memory management. Those
topics will be covered in the course.

> Rustの事前知識を前提としていませんが、少なくとも他のプログラミング言語についての知識があることを想定しています。
> また、システムプログラミングまたはメモリ管理の前提知識を想定していません。
> これらのトピックは、コース内でカバーします。

In other words, we'll be starting from scratch!\
You'll build up your Rust knowledge in small, manageable steps.
By the end of the course, you will have solved ~100 exercises, enough to
feel comfortable working on small to medium-sized Rust projects.

> つまり、スクラッチから始めます！
> 小さく、そして管理可能なステップを踏んで、Rustの知識を構築します。
> コースの最後では、小規模から中規模のRustプロジェクトで気持ちよく作業するために十分な、100個までの演習を解決するでしょう。

## Methodology（方法論）

This course is based on the "learn by doing" principle.\
It has been designed to be interactive and hands-on.

> 個のコースは、「実践で学ぶ」という原則に基づいています。
> それは、対話的で実践的になるように設計されています。

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this course
to be delivered in a classroom setting, over 4 days: each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
If you're interested in attending one of our training sessions, or if you'd like to
bring this course to your company, please [get in touch](https://mainmatter.com/contact/).

> Mainmatterは、クラスルームで4日間にわたって提供されるようにこのコースを設計しました。
> それぞれの参加者は、自分のペースでレッスンを進め、必要に応じて経験豊富なインストラクターがガイダンスを提供して、質問に回答して、トピックに深煎りします。
> トレーニングセッションに参加する興味がある場合、または個のコースを会社に持ち込みたい場合は、連絡を取ってください。

You can also follow the course on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
also find solutions to all exercises in the
[`solutions` branch of the GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions).

> また、独自でコースをフォローすることもできますが、行き詰まった場合に助けてくれる友人やメンターを探すことをおすすめします。
> また、GitHubリポジトリの`solutions`ブランチに、すべての演習の解答を見つけることもできます。

## Structure（構造）

On the left side of the screen, you can see that the course is divided into sections.
Each section introduces a new concept or feature of the Rust language.\
To verify your understanding, each section is paired with an exercise that you need to solve.

> 画面の左側で、コースがセクションに分割されていることを確認できます。
> それぞれのセクションは、Rust言語の新しい概念または機能を導入します。
> 理解を確認するために、それぞれのセクションは、解決しなくてはならない演習とペアになっています。

You can find the exercises in the
[companion GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust).\
Before starting the course, make sure to clone the repository to your local machine:

> GitHubリポジトリの手引書の中に演習を見つけることができます。
> コースを開始する前に、ローカルマシンにリポジトリをクローンしてください。

```bash
# If you have an SSH key set up with GitHub
git clone git@github.com:mainmatter/100-exercises-to-learn-rust.git
# Otherwise, use the HTTPS URL:
#
#   git clone https://github.com/mainmatter/100-exercises-to-learn-rust.git
```

We also recommend you work on a branch, so you can easily track your progress and pull
in updates from the main repository, if needed:

> また、必要に応じて、`main`リポジトリから容易に更新を追跡できるように、ブランチで作業することをおすすめします。

```bash
cd 100-exercises-to-learn-rust
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
Each exercise is structured as a Rust package.
The package contains the exercise itself, instructions on what to do (in `src/lib.rs`), and a test suite to
automatically verify your solution.

> すべての演習は、`exercises`フォルダに配置されています。
> それぞれの演習は、Rustパッケージとして構成されています。
> そのパッケージは、演習自体、何をするべきかの指示（`src/lib.rs`内）と、解答を自動確認するテストスイートを含んでいます。

### `wr`, the workshop runner（`wr`、ワークショップランナー）

To verify your solutions, we've provided a tool that will guide you through the course.
It is the `wr` CLI (short for "workshop runner").
Install it with:

> 解答を確認するために、コースをガイドするツールを提供しています。
> それは、`wr`CLI（"workshop runner"の略）です。
> それを次の通りインストールしてください。

```bash
cargo install --locked workshop-runner
```

In a new terminal, navigate back to the top-level folder of the repository.
Run the `wr` command to start the course:

> 新しいターミナルで、リポジトリの最上位フォルダーに移動してください。
> そして、コースを開始するために`wr`コマンドを実行してください。

```bash
wr
```

`wr` will verify the solution to the current exercise.\
Don't move on to the next section until you've solved the exercise for the current one.

> `wr`は現在の演習の解答を検証します。
> 現在の演習を解決するまで、次のセクションに進まないでください。

> We recommend committing your solutions to Git as you progress through the course,
> so you can easily track your progress and "restart" from a known point if needed.

> 必要に応じて、容易に進捗を追跡したり、理解している場所から「再開」できるように、コースを進めるにつれて、解答をコミットすることをおすすめします。

Enjoy the course!

> コースを楽しんでください！

## Author（著者）

This course was written by [Luca Palmieri](https://www.lpalmieri.com/), Principal Engineering
Consultant at [Mainmatter](https://mainmatter.com/rust-consulting/).\
Luca has been working with Rust since 2018, initially at TrueLayer and then at AWS.\
Luca is the author of ["Zero to Production in Rust"](https://zero2prod.com),
the go-to resource for learning how to build backend applications in Rust.\
He is also the author and maintainer of a variety of open-source Rust projects, including
[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef),
[Pavex](https://pavex.dev) and [`wiremock`](https://github.com/LukeMathWalker/wiremock-rs).

> 個のコースは、`Mainmatter`の主任エンジニアコンサルタントの`Luca Palmieri`によって記述されました。
> Lucaは2018年からRustを死傷しており、最初はTrueLayerで、その後はAWSで働いています。
> Lucaは、Rustでバックエンドアプリケーションを構築する方法を学ぶためのリソースである`Zero to Production in Rust`の著者です。
> また、彼は、`cargo-chef`、`Pavex`、`wiremock`などのさまざまなオープンソースのRustプロジェクトの著者およびメンテナーでもあります。
