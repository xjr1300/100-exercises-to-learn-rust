// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.
// これは`main.rs`ファイルで、従って`cargo`はバイナリーターゲットのルートとしてこれを解釈します。

// TODO: fix this broken import. Create a new library target in the `src` directory.
//   The library target should expose a public function named `hello_world` that takes no arguments
//   and returns nothing.
// この壊れたインポートを修正してください。`src`ディレクトリ内に新しいライブラリターゲットを作成してください。
// ライブラリーターゲットは、引数を受け取らず何も返さない`hello_world`と名付けたパブリック関数を公開します。
use packages::hello_world;

// This is the entrypoint of the binary.
// これはバイナリーのエントリポイントです。
fn main() {
    hello_world();
}
