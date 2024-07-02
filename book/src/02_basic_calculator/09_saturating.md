# Case-by-case behavior（状況に応じた振る舞い）

`overflow-checks` is a blunt tool: it's a global setting that affects the whole program.\
It often happens that you want to handle integer overflows differently depending on the context: sometimes
wrapping is the right choice, other times panicking is preferable.

> `overflow-checks`は遠慮のないツールです。
> それは、プログラム全体に影響を与えるグローバルな設定せす。
> コンテキストによって異なる整数オーバーフローを処理したいことがよくあります。
> 時々、包み込みは正しい選択で、他のときはパニックが好ましいです。

## `wrapping_` methods（wrapping_メソッド）

You can opt into wrapping arithmetic on a per-operation basis by using the `wrapping_` methods[^method].\
For example, you can use `wrapping_add` to add two integers with wrapping:

> `wrapping_`メソッドを使用して、操作ごとを基本にして包み込み算術を選択することができます。
> 例えば、包み込みを使用して2つの整数を加算するために、`wrapping_add`を使用できます。

```rust
let x = 255u8;
let y = 1u8;
let sum = x.wrapping_add(y);
assert_eq!(sum, 0);
```

## `saturating_` methods（saturating_メソッド）

Alternatively, you can opt into **saturating arithmetic** by using the `saturating_` methods.\
Instead of wrapping around, saturating arithmetic will return the maximum or minimum value for the integer type.
For example:

> 代わりに、`saturating_`メソッドを使用して**飽和算術**を選択することができます。
> 包み込みの代わりに、飽和算術は整数型の最大値または最小値を返します。
> 例えば・・・

```rust
let x = 255u8;
let y = 1u8;
let sum = x.saturating_add(y);
assert_eq!(sum, 255);
```

Since `255 + 1` is `256`, which is bigger than `u8::MAX`, the result is `u8::MAX` (255).\
The opposite happens for underflows: `0 - 1` is `-1`, which is smaller than `u8::MIN`, so the result is `u8::MIN` (0).

> `255 + 1`は`256`であり、`u8::MAX`より大きいため、結果は`u8::MAX`（255）です。
> アンダーフローの場合は逆です。`0 - 1`は`-1`であり、`u8::MIN`より小さいため、結果は`u8::MIN`（0）です。

You can't get saturating arithmetic via the `overflow-checks` profile setting—you have to explicitly opt into it
when performing the arithmetic operation.

> 飽和算術は`overflow-checks`プロファイル設定を介して取得できません。
> 算術操作を実行するときに明示的に選択する必要があります。

[^method]: You can think of methods as functions that are "attached" to a specific type.
We'll cover methods (and how to define them) in the next chapter.
特定の型に「付属した」関数としてメソッドを考えることができます。
次のチャプターでメソッド（およびそれらを定義する方法）について説明します。
