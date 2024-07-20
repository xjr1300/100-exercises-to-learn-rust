# Ownership（所有権）

If you solved the previous exercise using what this course has taught you so far,
your accessor methods probably look like this:

> このコースがこれまでに教えた方法を使用して、前の演習を解いた場合、アクセッサーメソッドはおそらく次のようになります。

```rust
impl Ticket {
    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}
```

Those methods compile and are enough to get tests to pass, but in a real-world scenario they won't get you very far.
Consider this snippet:

> それらのメソッドはコンパイルされ、テストにパスするのに十分ですが、それらは実際のシナリオで役に立ちません。
> 次のスニペットを考えてください。

```rust
if ticket.status() == "To-Do" {
    // We haven't covered the `println!` macro yet,
    // but for now it's enough to know that it prints
    // a (templated) message to the console
    // まだ`println!`マクロを説明していませんが、現時点では、コンソールに
    // テンプレート化したメッセージを出力することを理解するだけで十分です。
    println!("Your next task is: {}", ticket.title());
}
```

If you try to compile it, you'll get an error:

> そのコンパイルを試みると、エラーが発生します。

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`,
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

Congrats, this is your first borrow-checker error!

> おめでとうございます。これは最初の借用チェッカーのエラーです！

## The perks of Rust's ownership system（Rustの所有権システムの特典）

Rust's ownership system is designed to ensure that:

- Data is never mutated while it's being read
- Data is never read while it's being mutated
- Data is never accessed after it has been destroyed

> Rustの所有権システムは、次を確実にするために設計されています。
>
> - データが読み込まれている間に変更されない。
> - データが変更されている間に読み込まれない。
> - データが破壊された後でアクセスされない。

These constraints are enforced by the **borrow checker**, a subsystem of the Rust compiler,
often the subject of jokes and memes in the Rust community.

> これらの制約は、Rustコンパイラーのサブシステムである**借用チェッカー**によって強制され、Rustコミュニティー内でよくジョークやミームの対象となっています。

Ownership is a key concept in Rust, and it's what makes the language unique.
Ownership enables Rust to provide **memory safety without compromising performance**.
All these things are true at the same time for Rust:

1. There is no runtime garbage collector
2. As a developer, you rarely have to manage memory directly
3. You can't cause dangling pointers, double frees, and other memory-related bugs

> Rustにおいて、所有権は重要な概念であり、言語をユニークにします。
> 所有権は、Rustに**性能の妥協なしにメモリ安全**を提供します。
> 次のすべては、Rustにおいて同時に成立します。
>
> 1. ランタイムなガベージコレクターはありません。
> 2. 開発者として、まれに直接メモリを管理しなければならないとことがあります。
> 3. ダングリングポインター、二重解放、そして他のメモリに関連するバグを引き起こせません。

Languages like Python, JavaScript, and Java give you 2. and 3., but not 1.\
Language like C or C++ give you 1., but neither 2. nor 3.

> Python、JavaScript、そしてJavaのような言語は、２と3は成立しますが、1は成立しません。
> CやC++のような言語は、1は成立しますが、2と3は成立しません。

Depending on your background, 3. might sound a bit arcane: what is a "dangling pointer"?
What is a "double free"? Why are they dangerous?\
Don't worry: we'll cover these concepts in more details during the rest of the course.

> バックグラウンドに依存して、3は少し難解に聞こえるかもしれません。
> 「ダングリングポインター」とは何でしょうか？
> 「二重解放」とは何でしょうか？
> なぜそれらは危険なのでしょうか？
> 心配しないでください。コースの残りで、これらの概念をより詳細に説明します。

For now, though, let's focus on learning how to work within Rust's ownership system.

> しかし、現時点では、Rustの所有権システム内で作業する方法を学ぶことに焦点を当てましょう。

## The owner（所有者）

In Rust, each value has an **owner**, statically determined at compile-time.
There is only one owner for each value at any given time.

> Rustにおいて、それぞれの値は、コンパイル時に静的に決定される**所有者**を持ちます。
> 任意の時点でそれぞれの値には、たった1つの所有者がいます。

## Move semantics（ムーブセマンティック、ムーブの意味論）

Ownership can be transferred.

> 所有権は移動されます。

If you own a value, for example, you can transfer ownership to another variable:

> 例えば、値を所有している場合、他の変数に所有権を移動できます。

```rust
let a = "hello, world".to_string(); // <--- `a` is the owner of the String
                                    // <--- `a`は文字列の所有者です。
let b = a;  // <--- `b` is now the owner of the String
            // <--- 現在、`b`は文字列の所有者です。
```

Rust's ownership system is baked into the type system: each function has to declare in its signature
_how_ it wants to interact with its arguments.

> Rustの所有権システムは、型システム内に組み込まれています。
> それぞれの関数は、その引数と相互作用する方法を、そのシグネチャーで宣言しなければなりません。

So far, all our methods and functions have **consumed** their arguments: they've taken ownership of them.
For example:

> これまで、すべてのメソッドと関数は、それらの引数を**消費**しました。
> それらは、実引数の所有権を取りました。

```rust
impl Ticket {
    pub fn description(self) -> String {
        self.description
    }
}
```

`Ticket::description` takes ownership of the `Ticket` instance it's called on.\
This is known as **move semantics**: ownership of the value (`self`) is **moved** from the caller to
the callee, and the caller can't use it anymore.

> `Ticket::description`は、それを呼び出した`Ticket`インスタンスの所有権を取ります。
> これは、**ムーブセマンティック**として知られています。
> 値（`self`）の所有権は、呼び出し元から呼び出し先に**移動**して、呼び出し元はそれを使用できません。

That's exactly the language used by the compiler in the error message we saw earlier:

> それは、前に見たエラーメッセージでコンパイラーによって使用された正確な言語です。

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`,
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

In particular, this is the sequence of events that unfold when we call `ticket.status()`:

- `Ticket::status` takes ownership of the `Ticket` instance
- `Ticket::status` extracts `status` from `self` and transfers ownership of `status` back to the caller
- The rest of the `Ticket` instance is discarded (`title` and `description`)

> 特に、次は、`ticket.status()`を呼び出したときに、展開されたイベントのシーケンスです。
>
> - `Ticket::status`は、`Ticket`インスタンスの所有権を取ります。
> - `Ticket::status`は、`self`から`status`を抽出して、呼び出し元に`status`の所有権を移動します。
> - `Ticket`インスタンスの残りは、破棄されます（`title`と`description`）。

When we try to use `ticket` again via `ticket.title()`, the compiler complains: the `ticket` value is gone now,
we no longer own it, therefore we can't use it anymore.

> `ticket.title()`で再度`ticket`を使用することを試みると、コンパイラーは不満を言います。
> 現在`ticket`の値はなくなったため、もはやそれを所有しておらず、これ以上それを使用できません。

To build _useful_ accessor methods we need to start working with **references**.

> _有用な_アクセッサーメソッドを構築するために、**参照**を使用した作業を始める必要があります。

## Borrowing（借用）

It is desirable to have methods that can read the value of a variable without taking ownership of it.\
Programming would be quite limited otherwise. In Rust, that's done via **borrowing**.

> 値の所有権を取らずに、変数の値を読み込むことができるメソッドを持つことが望ましいです。
> そうでなければ、プログラミングは、かなり制限されます。
> Rustでは、それを**借用**を介して行います。

Whenever you borrow a value, you get a **reference** to it.\
References are tagged with their privileges[^refine]:

- Immutable references (`&`) allow you to read the value, but not to mutate it
- Mutable references (`&mut`) allow you to read and mutate the value

> 値を借用するときはいつでも、その**参照**を取得します。
> 参照は、それらの権限でタグ付けされます。
>
> - 不変参照（`&`）は、値を読み込めるようにしますが、それを変更できません。
> - 可変参照（`&mut`）は、値を読み込み、変更できるようにします。

Going back to the goals of Rust's ownership system:

- Data is never mutated while it's being read
- Data is never read while it's being mutated

> Rustの所有権システムの目的に戻ります。
>
> - データは、それが読み込まれている間、変更できません。
> - データは、それが変更されている間、読み込みできません。

To ensure these two properties, Rust has to introduce some restrictions on references:

- You can't have a mutable reference and an immutable reference to the same value at the same time
- You can't have more than one mutable reference to the same value at the same time
- The owner can't mutate the value while it's being borrowed
- You can have as many immutable references as you want, as long as there are no mutable references

> これら2つの属性を確実にするために、Rustは参照にいくつかの制約を導入しなければなりません。
>
> - 同時に同じ値への可変参照と不変参照を持つことはできません。
> - 同時に同じ値への可変参照を複数持つことはできません（1つしか持てない）。
> - 所有者は、値が借用されている間、値を変更できません。
> - 可変参照がない限り、希望に応じて多くの不変参照を持てます。

In a way, you can think of an immutable reference as a "read-only" lock on the value,
while a mutable reference is like a "read-write" lock.

> ある意味で、値の「読み込み専用」ロックとして不変参照を考えることができる一方で、可変参照は「読み書き」ロックのようです。

All these restrictions are enforced at compile-time by the borrow checker.

> これらすべての制約は、借用チェッカーによってコンパイル時に強制されます。

### Syntax（構文）

How do you borrow a value, in practice?\
By adding `&` or `&mut` **in front a variable**, you're borrowing its value.
Careful though! The same symbols (`&` and `&mut`) in **front of a type** have a different meaning:
they denote a different type, a reference to the original type.

For example:

> 実際に、どのように値を借用すればよいのでしょうか？
> **変数の前**に`&`または`&mut`を追加することで、その値を借用できます。
> ただし、注意してください！
> **型の前**の同じシンボル（`&`と`&mut`）は、異なる意味を持ちます。
> それらは、オリジナルの型への参照を示す異なる型です。
>
> 例えば・・・

```rust
struct Configuration {
    version: u32,
    active: bool,
}

fn main() {
    let config = Configuration {
        version: 1,
        active: true,
    };
    // `b` is a reference to the `version` field of `config`.
    // The type of `b` is `&u32`, since it contains a reference to a `u32` value.
    // We create a reference by borrowing `config.version`, using the `&` operator.
    // Same symbol (`&`), different meaning depending on the context!
    // `b`は、`config`の`version`フィールドへの参照です。
    // `b`は`u32`値への参照を含むため、`b`の型は`&u32`です。
    // `&`演算子を使用して`config.version`を借用することで、参照を作成しました。
    // 同じシンボル（`&`）ですが、文脈に依存して異なる意味を持ちます！
    let b: &u32 = &config.version;
    //     ^ The type annotation is not necessary,
    //       it's just there to clarify what's going on
    //       型注釈は必要ありませんが、それは何が起こっているかを明確にするためだけにあります。
}
```

The same concept applies to function arguments and return types:

> 同じ概念は、関数の引数と戻り値の型に適用されます。

```rust
// `f` takes a mutable reference to a `u32` as an argument,
// bound to the name `number`
// `f`は、引数として`u32`への可変参照を取り、名前`number`に拘束されます。
fn f(number: &mut u32) -> &u32 {
    // [...]
}
```

## Breathe in, breathe out（息を吸って、息を吐く）

Rust's ownership system can be a bit overwhelming at first.\
But don't worry: it'll become second nature with practice.\
And you're going to get a lot of practice over the rest of this chapter, as well as the rest of the course!
We'll revisit each concept multiple times to make sure you get familiar with them
and truly understand how they work.

> Rustの所有権システムは、最初は少し打ちのめさせられるかもしれません。
> しかし、心配しないでください。それは実践により自然になります。
> そして、コースの残りと同様に、この章の残りで多くの実践を行います。
> それらに慣れることを確実にするために、何回もそれぞれの概念を再訪して、それらがどのように機能するかを本当に理解します。

Towards the end of this chapter we'll explain _why_ Rust's ownership system is designed the way it is.
For the time being, focus on understanding the _how_. Take each compiler error as a learning opportunity!

> この章の最後に、Rustの所有権システムが、そのように設計された理由を説明します。
> 現時点では、_どのように_理解するかに焦点を当ててください。
> 学ぶ機会として、それぞれのコンパイルエラーを受け入れてください！

[^refine]: This is a great mental model to start out, but it doesn't capture the _full_ picture.
We'll refine our understanding of references [later in the course](../07_threads/06_interior_mutability.md).
これは始めるための素晴らしいメンタルモデルですが、_完全な_姿を捉えていません。
コースの後半で、参照の理解を洗練させます。
