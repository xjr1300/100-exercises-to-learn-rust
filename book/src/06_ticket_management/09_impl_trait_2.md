# `impl Trait` in argument position（引数の位置にあるimpl Trait）

In the previous section, we saw how `impl Trait` can be used to return a type without specifying its name.\
The same syntax can also be used in **argument position**:

> 前の節において、その名前を指定しないで型を返すために`impl Trait`を使用する方法を確認しました。
> 同じ構文は、**引数の位置**でも使用できます。

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` takes an iterator of `i32`s and prints each element.\
When used in **argument position**, `impl Trait` is equivalent to a generic parameter with a trait bound:

> `print_iter`は、`i32`のイテレーターを受け取り、それぞれの要素を出力します。
> **引数の位置**で使用された場合、`impl Trait`はトレイト境界を持つジェネリックなパラメーターと同じです。

```rust
fn print_iter<T>(iter: T)
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## Downsides（欠点）

As a rule of thumb, prefer generics over `impl Trait` in argument position.\
Generics allow the caller to explicitly specify the type of the argument, using the turbofish syntax (`::<>`),
which can be useful for disambiguation. That's not the case with `impl Trait`.

> 経験則として、引数の位置に`impl Trait`よりもジェネリックを好むようにしてください。
> ジェネリックは、ターボフィッシュ構文（`::<>`）を使用して、呼び出し側に引数の型を明示的に指定できるようにして、曖昧さを解消するために役立ちます。
> それは`impl Trait`を使用したケースには当てはまりません。
