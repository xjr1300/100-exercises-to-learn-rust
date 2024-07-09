// TODO: Add `anyhow` as a dependency of this project.
//  Don't touch this import!
// このプロジェクトの依存関係として、`anyhow`を追加してください。
// 次のインポートに触ってはいけません。

// When you import a type (`Error`) from a dependency, the import path must start
// with the crate name (`anyhow`, in this case).
// 依存関係から`Error`型をインポートしたとき、そのインポートパスはクレート名で開始しなければ成りません（この場合は`anyhow`です）。
use anyhow::Error;
