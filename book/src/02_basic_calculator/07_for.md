# Loops, part 2: `for`（ループ、その2: for）

Having to manually increment a counter variable is somewhat tedious. The pattern is also extremely common!\
To make this easier, Rust provides a more concise way to iterate over a range of values: the `for` loop.

> 手動でカウンター変数をインクリメントしなければならないことは、なにか面倒です。このパターンはとても一般的です！
> これを簡単にするために、Rustは、値の範囲を反復処理するためのより簡潔な方法を提供しています。それは`for`ループです。

## The `for` loop（forループ）

A `for` loop is a way to execute a block of code for each element in an iterator[^iterator].
Here's the general syntax:

> `for`ループは、イテレーター内のそれぞれの要素でコードブロックを実行する方法です。
> ここに一般的な構文を示します。

```rust
for <element> in <iterator> {
    // code to execute
}
```

## Ranges（レンジ、範囲）

Rust's standard library provides **range** type that can be used to iterate over a sequence of numbers[^weird-ranges].
For example, if we want to sum the numbers from 1 to 5:

> Rust標準ライブラリは、数のシーケンスを反復処理するために使用できる**レンジ**型を提供しています。
> 例えば、1から5までの数を合計したいとします。

```rust
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}
```

Every time the loop runs, `i` will be assigned the next value in the range before executing the block of code.
There are five kinds of ranges in Rust:

- `1..5`: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.
- `1..=5`: An inclusive range. It includes all numbers from 1 to 5. It includes the last value, 5.
- `1..`: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).
- `..5`: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.
- `..=5`: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.

> ループが実行されるたびに、`i`はコードブロックを実行する前に、レンジ内の次の値が割り当てられます。
> Rustには次の5種類のレンジがあります。
>
> - `1..5`: 左閉右開レンジです。それは1から4までのすべての値を含みます。それは最後の値である5を含みません。
> - `1..=5`: 包括的（左閉右閉）なレンジです。それは1から5までのすべての値を含みます。それは最後の値である5を含みます。
> - `1..`: 最後が開かれたレンジです。それは1から無限までのすべての値を含みます（といっても、その整数型の最大値までです）。
> - `..5`: その整数型の最小値から開始して4で終わるレンジです。それは最後の値である5を含みません。
> - `..=5`: その整数型の最小値から開始して5で終わるレンジです。それは最後の値である5を含みます。

You can use a `for` loop with the first three kinds of ranges, where the starting point
is explicitly specified. The last two range types are used in other contexts, that we'll cover later.
The extreme values of a range don't have to be integer literals—they can be variables or expressions too!
For example:

> レンジの最初の3種類を`for`ループで使用でき、開始値源は明示的に指定されています。
> 最後の2つのレンジタイプは、他のコンテキストで使用され、それを後で説明します。
> レンジの極端な値は、整数リテラルである必要はなく、それらは変数や式でもかのうです！

```rust
let end = 5;
let mut sum = 0;

for i in 1..(end + 1) {
    sum += i;
}
```

## Further reading（参考資料）

- [`for` loop documentation](https://doc.rust-lang.org/std/keyword.for.html)

[^iterator]: Later in the course we'll give a precise definition of what counts as an "iterator".
For now, think of it as a sequence of values that you can loop over.
コースの後の方で、「イテレーター」とは何を示すか、正確な定義を与えます。
現時点では、それをループ可能な値のシーケンスと考えてください。

[^weird-ranges]: You can use ranges with other types too (e.g. characters and IP addresses),
but integers are definitely the most common case in day-to-day Rust programming.
例えば文字やIPアドレスなど、他の型でレンジを使用できますが、整数が絶対的に日常のRustプログラミングで最も一般的なケースです。
