# Destructors（デストラクター）

When introducing the heap, we mentioned that you're responsible for freeing the memory you allocate.\
When introducing the borrow-checker, we also stated that you rarely have to manage memory directly in Rust.

> ヒープを紹介したとき、割り当てたメモリを解放する責任があることを言及しました。
> 借用チェッカーを紹介したとき、Rustでは、メモリを直接管理する必要がほとんどないことも述べました。

These two statements might seem contradictory at first.
Let's see how they fit together by introducing **scopes** and **destructors**.

> 最初は、2つの文が矛盾しているように見えるかもしれません。
> **スコープ**と**デストラクター**を導入して、それらがどのように適合するか確認しましょう。

## Scopes（スコープ）

The **scope** of a variable is the region of Rust code where that variable is valid, or **alive**.

> 変数の**スコープ**は、その変数が有効または**生きている**Rustコードの領域です。

The scope of a variable starts with its declaration.
It ends when one of the following happens:

1. the block (i.e. the code between `{}`) where the variable was declared ends

   ```rust
   fn main() {
      // `x` is not yet in scope here
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x's scope starts here...
      let h = "!".to_string(); //   |
   } //  <-------------- ...and ends here
   ```

2. ownership of the variable is transferred to someone else (e.g. a function or another variable)

   ```rust
   fn compute(t: String) {
      // Do something [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s's scope starts here...
                   //                    |
       compute(s); // <------------------- ..and ends here
                   //   because `s` is moved into `compute`
   }
   ```

> 変数のスコープは、その宣言で始まります。
> それは、次のうち1つが発生すると終わります。
>
> 1. その変数が宣言された、例えば`{}`間のコードなどのブロックが終了する。
> 2. 変数の所有権が、例えば関数または他の変数など、他の誰かに移動される。

## Destructors（デストラクター）

When the owner of a value goes out of scope, Rust invokes its **destructor**.\
The destructor tries to clean up the resources used by that value—in particular, whatever memory it allocated.

> 値の所有者がスコープ外になると、Rustはその**デストラクター**を呼び出します。
> デストラクターは、その値によって使用されたリソース、特に割り当てられたメモリをクリーンアップすることを試みます。

You can manually invoke the destructor of a value by passing it to `std::mem::drop`.\
That's why you'll often hear Rust developers saying "that value has been **dropped**" as a way to state that a value
has gone out of scope and its destructor has been invoked.

> 値を`std::mem::drop`に渡すことで、値のデストラクターを手動で呼び出すことができます。
> Rustの開発者が、値がスコープ外になり、そのデストラクターが呼び出されたことを述べる方法として、「その値は**ドロップ**されました」と言うことをよく聞くでしょう。

### Visualizing drop points（ドロップポイントの可視化）

We can insert explicit calls to `drop` to "spell out" what the compiler does for us. Going back to the previous example:

> コンパイラーがしてくれることを「記述して伝える」ために、明示的な`drop`呼び出しを挿入できます。
> 前の例に戻りましょう。

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

It's equivalent to:

> 上記は次と等価です。

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // Variables are dropped in reverse order of declaration
   // 変数は、宣言の逆順でドロップされます。
   drop(h);
   drop(x);
   drop(y);
}
```

Let's look at the second example instead, where `s`'s ownership is transferred to `compute`:

> 代わりに、`s`の所有権が`compute`に移動した、2つ目の例を確認しましょう。

```rust
fn compute(s: String) {
   // Do something [...]
   // なにかします。
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

It's equivalent to this:

> それは次と等価です。

```rust
fn compute(t: String) {
    // Do something [...]
    // なにかします。
    drop(t); // <-- Assuming `t` wasn't dropped or moved
             //     before this point, the compiler will call
             //     `drop` here, when it goes out of scope
             //    この時点まで`t`がドロップされたり移動されていないと仮定して、
             //    それがスコープ外になったとき、コンパイラーはここで｀drop`を呼び出します。
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

Notice the difference: even though `s` is no longer valid after `compute` is called in `main`, there is no `drop(s)`
in `main`.
When you transfer ownership of a value to a function, you're also **transferring the responsibility of cleaning it up**.

> 違いに注意してください。`main`内で`compute`が呼び出された後、`s`はもはや有効ではありませんが、`main`内に`drop(s)`はありません。
> 関数に値の所有権を移動したとき、**それをクリーンアップする責任も移動**します。

This ensures that the destructor for a value is called **at most[^leak] once**, preventing
[double free bugs](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) by design.

> 値のためのデストラクターは、設計によって二重解放のバグを避けるために、**最大で一度**呼び出されます。

### Use after drop（ドロップのあとで使用する）

What happens if you try to use a value after it's been dropped?

> 値がドロップされた後で、値を使用することを試みると何が発生するでしょうか？

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

If you try to compile this code, you'll get an error:

> このコードをコンパイルすることを試みると、次のエラーが発生します。

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

Drop **consumes** the value it's called on, meaning that the value is no longer valid after the call.\
The compiler will therefore prevent you from using it, avoiding [use-after-free bugs](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).

> ドロップは、それが呼ばれた値を**消費する**ため、呼び出した後、その値はもはや有効ではないことを意味します。
> よって、コンパイラーは、解放後に使用するバグを回避するために、それを使用することを防ぎます。

### Dropping references（参照のドロップ）

What if a variable contains a reference?\
For example:

> 変数が参照を含んでいた場合はどうなるでしょうか？
> 例えば・・・

```rust
let x = 42i32;
let y = &x;
drop(y);
```

When you call `drop(y)`... nothing happens.\
If you actually try to compile this code, you'll get a warning:

> `drop(y)`を呼び出したとき・・・何も発生しません。
> 実際にこのコードをコンパイルすることを試みた場合、警告が発生します。

```text
warning: calls to `std::mem::drop` with a reference
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

It goes back to what we said earlier: we only want to call the destructor once.\
You can have multiple references to the same value—if we called the destructor for the value they point at
when one of them goes out of scope, what would happen to the others?
They would refer to a memory location that's no longer valid: a so-called [**dangling pointer**](https://en.wikipedia.org/wiki/Dangling_pointer),
a close relative of [**use-after-free bugs**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).
Rust's ownership system rules out these kinds of bugs by design.

> これは、前述のことに戻ります：デストラクターを一度だけ呼び出したいだけです。
> 同じ値への複数の参照を持つことができます。それらの1つがスコープ外になったときにその値のデストラクターを呼び出した場合、他に何が発生するでしょうか？
> それらは、ダングリングポインターとも呼ばれる、もはや有効でないメモリ位置を参照して、解放した後に使用するバグに密接に関連します。
> Rustの所有権システムは、設計によってこれらの種類のバグを排除します。

[^leak]: Rust doesn't guarantee that destructors will run. They won't, for example, if
you explicitly choose to [leak memory](../07_threads/03_leak.md).
Rustは、デストラクターが実行されることを保証しません。
例えば、メモリリークを明示的に選択した場合、デストラクターの呼び出しはされません。
