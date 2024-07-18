# Libraries and binaries（ライブラリとバイナリ）

It took a bit of code to implement the `Error` trait for `TicketNewError`, didn't it?\
A manual `Display` implementation, plus an `Error` impl block.

> `TicketNewError`に対して`Error`トレイトを実装するために、少しのコードが必要でしたか？
> 手動の`Display`の実装は、`Error`の`impl`ブロックに加えられました。

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.\
But we're getting ahead of ourselves: `thiserror` is a third-party crate, it'd be our first dependency!

> カスタムエラー型の作成を簡素化する**手続きマクロ**を提供するRustのクレートである`thiserror`を使用して、いくつかのボイラープレートを削除できます。
> しかし、私たちは先走っています。`thiserror`はサードパーティのクレートであり、これが最初の依存関係になります！

Let's take a step back to talk about Rust's packaging system before we dive into dependencies.

> 依存関係に深入りする前にRustのパッケージシステムについて話すために、一歩後退しましょう。

## What is a package?（パッケージとは？）

A Rust package is defined by the `[package]` section in a `Cargo.toml` file, also known as its **manifest**.
Within `[package]` you can set the package's metadata, such as its name and version.

> Rustパッケージは、`Cargo.toml`ファイル内の`[package]`節によって定義され、**マニフェスト**としても知られています。
> `[package]`内で、パッケージの名前やバージョンなどのメタデータも設定できます。

Go check the `Cargo.toml` file in the directory of this section's exercise!

> この節の演習ディレクトリ内の`Cargo.toml`ファイルを確認してください。

## What is a crate?（クレートとは？）

Inside a package, you can have one or more **crates**, also known as **targets**.\
The two most common crate types are **binary crates** and **library crates**.

> パッケージの内部で、**ターゲット**とも知られる1つ以上の**クレート**を持つことができます。
> 2つの最も一般的なクレートのタイプは、**バイナリークレート**と**ライブラリクレート**です。

### Binaries（バイナリー）

A binary is a program that can be compiled to an **executable file**.\
It must include a function named `main`—the program's entry point. `main` is invoked when the program is executed.

> バイナリーは、**実行ファイル**にコンパイルされるプログラムです。
> それは、プログラムのエントリポイントである`main`と名付けられた関数を含まなくてはなりません。
> `main`は、プログラムが実行されたときに呼び出されます。

### Libraries（ライブラリ）

Libraries, on the other hand, are not executable on their own. You can't _run_ a library,
but you can _import its code_ from another package that depends on it.\
A library groups together code (i.e. functions, types, etc.) that can be leveraged by other packages as a **dependency**.

> 一方、ライブラリはそれ自身で実行可能ではありません。ライブラリを**実行**することはできませんが、それに依存する他のパッケージから**そのコードをインポート**できます。
> ライブラリは、**依存関係**として他のパッケージによって利用されるコード（例えば、関数、型など）を一緒にグループ化します。

All the exercises you've solved so far have been structured as libraries, with a test suite attached to them.

> これまでに解いてきたすべての演習は、それらに付属するテストスイートと一緒に、ライブラリとして構成されています。

### Conventions（慣例）

There are some conventions around Rust packages that you need to keep in mind:

- The package's source code is usually located in the `src` directory.
- If there's a `src/lib.rs` file, `cargo` will infer that the package contains a library crate.
- If there's a `src/main.rs` file, `cargo` will infer that the package contains a binary crate.

> Rustパッケージのまわりに、気に留めておくべき慣例がいくつかあります。
>
> - パッケージのソースコードは、通常`src`ディレクトリ内に配置されます。
> - `src/lib.rs`ファイルが存在する場合、`cargo`はパッケージがライブラリクレートを含んでいると推測します。
> - `src/main.rs`ファイルが存在する場合、`cargo`はパッケージがバイナリークレートを含んでいると推測します。

You can override these defaults by explicitly declaring your targets in the `Cargo.toml` file—see
[`cargo`'s documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets) for more details.

> `Cargo.toml`ファイルで明示的にターゲットを宣言することによって、これらのデフォルトを上書きできます。
> 詳細は`cargo`のドキュメントを参照してください。

Keep in mind that while a package can contain multiple crates, it can only contain one library crate.

> パッケージは複数のクレートを含むことができる一方で、パッケージはたった1つのライブラリクレートのみを含むことができます。

## Scaffolding a new package（新しいパッケージの足場）

You can use `cargo` to scaffold a new package:

> 新しいパッケージの足場とするために`cargo`を使用できます。

```bash
cargo new my-binary
```

This will create a new folder, `my-binary`, containing a new Rust package with the same name and a single
binary crate inside. If you want to create a library crate instead, you can use the `--lib` flag:

> これは、`my-binary`という新しいフォルダを作成して、内部に同じ名前で単独のバイナリークレートを持つ新しいRustパッケージを含めます。
> 代わりにライブラリクレートを作成する場合、`--lib`フラグを使用できます。

```bash
cargo new my-library --lib
```
