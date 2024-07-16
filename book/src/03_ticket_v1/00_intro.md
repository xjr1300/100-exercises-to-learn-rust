# Modelling A Ticket（チケットのモデリング）

The first chapter should have given you a good grasp over some of Rust's primitive types, operators and
basic control flow constructs.\
In this chapter we'll go one step further and cover what makes Rust truly unique: **ownership**.\
Ownership is what enables Rust to be both memory-safe and performant, with no garbage collector.

> 最初の章は、Rustのプリミティブ型、演算子そして基本的な制御フローの構築についての良い理解を与えるはずです。
> この章は、先にステップを進めて、Rustを本当にユニークにする**所有権**について説明します。
> 所有権は、メモリセーフと性能の両方を有効にするもので、ガベージコレクターを持ちません。

As our running example, we'll use a (JIRA-like) ticket, the kind you'd use to track bugs, features, or tasks in
a software project.\
We'll take a stab at modeling it in Rust. It'll be the first iteration—it won't be perfect nor very idiomatic
by the end of the chapter. It'll be enough of a challenge though!\
To move forward you'll have to pick up several new Rust concepts, such as:

- `struct`s, one of Rust's ways to define custom types
- Ownership, references and borrowing
- Memory management: stack, heap, pointers, data layout, destructors
- Modules and visibility
- Strings

> 実行する例として、ソフトウェアプロジェクトのバグ、機能またはタスクを追跡するために使用する、JIRAのようなチケットを使用します。
> Rustでモデリングします。それは最初のイテレーションで、章の最後でもそれは完全で理想的ではありません。
> しかし、それは挑戦するには十分です！
> 先に進むために、次のようなRustの新しい概念を取り上げなければなりません。
>
> - `struct`は、カスタム型を定義するRustの1つの方法です。
> - 所有権、参照そして借用
> - メモリ管理: スタック、ヒープ、ポインター、データレイアウト、デストラクター
> - モジュールと可視性
> - 文字列
