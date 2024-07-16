# Wrapping up（仕上げ）

When it comes to domain modelling, the devil is in the details.\
Rust offers a wide range of tools to help you represent the constraints of your domain directly in the type system,
but it takes some practice to get it right and write code that looks idiomatic.

> ドメインモデリングにおいて、悪魔は詳細に宿ります。
> Rustは、型システム内に直接ドメインの一貫性を表現するための幅広いツールを提供していますが、
> 正しく理解して慣用的なコードを記述するためには、ある程度の実践が必要です。

Let's close the chapter with one final refinement of our `Ticket` model.\
We'll introduce a new type for each of the fields in `Ticket` to encapsulate the respective constraints.\
Every time someone accesses a `Ticket` field, they'll get back a value that's guaranteed to be valid—i.e. a
`TicketTitle` instead of a `String`. They won't have to worry about the title being empty elsewhere in the code:
as long as they have a `TicketTitle`, they know it's valid **by construction**.

> `Ticket`モデルの最後の改良で章を終了しましょう。
> それぞれの制約をカプセル化するために`Ticket`内のそれぞれのフィールドに対して新しい型を導入します。
> 誰かが`Ticket`のフィールドにアクセスするたびに、それらは有効と保証された値、例えば`String`の代わりに`TicketTitle`を返します。
> コードの何処かでタイトルがからであることを心配する必要はありません。
> `TicketTitle`がある限り、**構築によって**有効であることがわかります。

This is just an example of how you can use Rust's type system to make your code safer and more expressive.

> これは、ちょうどコードを安全そしてより表現を豊かにするに、Rustの型システムを使用する方法を示す例です。

## Further reading（参考資料）

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
