# Combinators（コンビネーター）

Iterators can do so much more than `for` loops!\
If you look at the documentation for the `Iterator` trait, you'll find a **vast** collections of
methods that you can leverage to transform, filter, and combine iterators in various ways.

> イテレーターは`for`ループ以上のことができます！
> `Iterator`トレイトのドキュメントを確認した場合、さまざまな方法でイテレーターを変換、フィルターそして結合して利用するメソッドの**膨大な**コレクションを見つけるでしょう。

Let's mention the most common ones:

- `map` applies a function to each element of the iterator.
- `filter` keeps only the elements that satisfy a predicate.
- `filter_map` combines `filter` and `map` in one step.
- `cloned` converts an iterator of references into an iterator of values, cloning each element.
- `enumerate` returns a new iterator that yields `(index, value)` pairs.
- `skip` skips the first `n` elements of the iterator.
- `take` stops the iterator after `n` elements.
- `chain` combines two iterators into one.

> 最も一般的なものを言及しましょう。
>
> - `map`は、イテレーターのそれぞれの要素に関数を適用します。
> - `filter`は、述語を満足する要素のみを保持します。
> - `filter_map`は、1ステップで`filter`と`map`を組み合わせます。
> - `cloned`は、それぞれの要素をクローンして、参照のイテレーターを値のイテレーターに変換します。
> - `enumerate`は、`(index, value)`のペアを生成する新しいイテレーターを返します。
> - `skip`はイテレーターの最初の`n`要素をスキップします。
> - `take`は、`n`要素後にイテレーターを停止します。
> - `chain`は、2つのイテレーターを1つに結合します。

These methods are called **combinators**.\
They are usually **chained** together to create complex transformations in a concise and readable way:

> これらのメソッドは**コンビネーター**と呼ばれます。
> それらは、通常、簡潔で読みやすい方法で、複雑な変換を作成するために一緒に**連鎖**されます。

```rust
let numbers = vec![1, 2, 3, 4, 5];
// The sum of the squares of the even numbers
// 偶数の二乗の合計
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

## Closures（クロージャー）

What's going on with the `filter` and `map` methods above?\
They take **closures** as arguments.

> 上記の`filter`と`map`メソッドは何をしているのでしょうか？
> それらは引数として**クロージャー**を受け取ります。

Closures are **anonymous functions**, i.e. functions that are not defined using the `fn` syntax we are used to.\
They are defined using the `|args| body` syntax, where `args` are the arguments and `body` is the function body.
`body` can be a block of code or a single expression.
For example:

> クロージャーは**匿名関数**です。つまり、使用してきた`fn`構文を使用して定義されない関数です。
> それらは`|args| body`構文を使用して定義され、`args`は引数で`body`は関数の本体です。
> `body`はコードのブロックまたは、単独の式になります。
> 例えば・・・

```rust
// An anonymous function that adds 1 to its argument
// 引数に1を足す匿名関数
let add_one = |x| x + 1;
// Could be written with a block too:
// ブロックで記述することもできます。
let add_one = |x| { x + 1 };
```

Closures can take more than one argument:

> クロージャーは1つ以上の引数を受け取ることができます。

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

They can also capture variables from their environment:

> それらは、それらの環境から変数をキャプチャーすることもできます。

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

If necessary, you can specify the types of the arguments and/or the return type:

> 必要に応じて、引数の型及び戻り値の型を指定できます。

```rust
// Just the input type
// 単なる引数の型です。
let add_one = |x: i32| x + 1;
// Or both input and output types, using the `fn` syntax
// `fn`構文を使用した、引数と出力の型の両方です。
let add_one: fn(i32) -> i32 = |x| x + 1;
```

## `collect`

What happens when you're done transforming an iterator using combinators?\
You either iterate over the transformed values using a `for` loop, or you collect them into a collection.

> コンビネーターを使用してイテレータを変換した後、何が起こるでしょうか？
> `for`ループを使用して変換された値を反復処理することも、コレクション内にそれらを集めることもできます。

The latter is done using the `collect` method.\
`collect` consumes the iterator and collects its elements into a collection of your choice.

> 後者は、`collect`メソッドを使用して行われます。
> `collect`はイテレーターを消費して、選択したコレクション内にその要素を集めます。

For example, you can collect the squares of the even numbers into a `Vec`:

> 例えば、`Vec`に偶数の二乗を集めることができます。

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` is generic over its **return type**.\
Therefore you usually need to provide a type hint to help the compiler infer the correct type.
In the example above, we annotated the type of `squares_of_evens` to be `Vec<u32>`.
Alternatively, you can use the **turbofish syntax** to specify the type:

> `collect`は、その**戻り値の型**に対してジェネリックです。
> 従って、通常、コンパイラーが正確な型を推論することを助けるために、型ヒントを提供する必要があります。
> 上記の例において、`squares_of_evens`の型が`Vec<u32>`になることを注釈しました。
> 代わりに、型を指定するために**ターボフィッシュ構文**を使用できます。

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // Turbofish syntax: `<method_name>::<type>()`
    // It's called turbofish because `::<>` looks like a fish
    // ターボフィッシュ構文: `<メソッド名>::<型>()`
    // `::<>`が魚のように見えるため、ターボフィッシュと呼ばれます。
    .collect::<Vec<u32>>();
```

## Further reading（参考資料）

- [`Iterator`'s documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html) gives you an
  overview of the methods available for iterators in `std`.
- [The `itertools` crate](https://docs.rs/itertools/) defines even **more** combinators for iterators.
