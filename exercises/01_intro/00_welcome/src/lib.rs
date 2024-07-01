// This is a Rust file. It is a plain text file with a `.rs` extension.
// これはRustのファイルです。それは`.rs`拡張子が付いたプレーンなテキストファイルです。
//
// Like most modern programming languages, Rust supports comments. You're looking at one right now!
// Comments are ignored by the compiler; you can leverage them to annotate code with notes and
// explanations.
// There are various ways to write comments in Rust, each with its own purpose.
// For now we'll stick to the most common one: the line comment.
// Everything from `//` to the end of the line is considered a comment.
// 最も現代的なプログラミング言語のように、Rustはコメントをサポートします。あなたはすぐに確認します！
// Rustにおいて、コメントを記述方法はいくつかあり、それぞれは独自の目的を持ちます。
// 今のところ、最も一般的なラインコメントに固執します。
// 行の`//`から最後までの全ては、コメントとして考えられます。

// Exercises will include `TODO`, `todo!()` or `__` markers to draw your attention to the lines
// where you need to write code.
// You'll need to replace these markers with your own code to complete the exercise.
// Sometimes it'll be enough to write a single line of code, other times you'll have to write
// longer sections.
// 演習は、コードを記述する必要がある行に注目を引き付けるために、`TODO`、`todo!()`または`__`マーカーを含んでいます。
// 演習を完成するために独自のコードでこれらのマーカーを置き換える必要があります。
// 時々、1行のコードを記述することで十分であり、あるときは、長いセクションを記述しなくてはならないでしょう。
//
// If you get stuck for more than 10 minutes on an exercise, grab a trainer! We're here to help!
// You can also find solutions to all exercises in the `solutions` git branch.
// 演習で10分より多く立ち止まった場合、トレーナーを得ましょう。支援するためにここにいます。
// また、gitブランチの`solutions`内にすべての演習の解答を見つけることができます。
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn Rust!"
}

// Your solutions will be automatically verified by a set of tests.
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// from the root of this exercise's directory. That's what the `wr` command does for you
// under the hood.
// 解答は、一連のテストによって自動的に検証されます。
// ターミナル内で、この演習のディレクトリのルートから`cargo test`コマンドを呼び出すことによって、直接これらのテストを実行できます。
// それは、`wr`コマンドが内部で行うことです。
//
// Rust lets you write tests alongside your code.
// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
// You'll learn more about attributes and testing later in the course.
// For now, just know that you need to look for the `#[cfg(test)]` attribute to find the tests
// that will be verifying the correctness of your solutions!
// Rustは、コードの側にテストを記述できるようにします。
// `#[cfg(test)]`属性は、例えば、`cargo test`を実行してい、テストを実行したときに下のコードをただコンパイルすることを、コンパイラーに伝えます。
// 後の方のコースで属性とテストについてより学習するでしょう。
// ここでは、単純に 系統の正確性を検証するテストを探すために`#[cfg(test)]`属性を探す必要があることを理解してください。
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// tested, not the tests themselves.
// ⚠ **テストを修正しない** ⚠
// それらは、解答を検証する支援するためにそこにあります。テストされるコードのみを変更しなければならず、テスト自身ではありません。
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
