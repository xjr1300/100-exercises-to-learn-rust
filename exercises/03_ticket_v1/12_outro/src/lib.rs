// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
// 新しい`Order`型を定義してください。
// それは、`product_name`、`quantity`そして`unit_price`の3つの情報の追跡を続けます。
// 製品名は空になってはならず、またそれは300バイトより大きくなりません。
// 数量は、ゼロよりも大きくなければなりません。
// 単価は、セント単位で、ゼロよりも大きくなければなりません。
// 注文は、注文の合計額を返す`total`と名付けたメソッドを含まなければなりません。
// 注文は、それぞれのフィールドのセッターとゲッターを提供しなければなりません。
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
// 今回、テストは異なる場所に配置されています。`tests`フォルダー無いです。
// `tests`フォルダーは、`cargo`にとって特別な場所です。それは、それが**統合テスト**を探す場所です。
// ここの強盗は、とても特別な意味を持っています。それらはプロジェクトの**公開されたAPI**をテストします。
// 型とメソッドの可視性に注意を払う必要があります。
// 統合テストは、プライベートまたは`pub(crate)`アイテムにアクセスできません。

pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

fn clean_product_name(s: String) -> String {
    let s = s.trim();
    if s.is_empty() {
        panic!("product name should not be empty")
    }
    if 300 < s.as_bytes().len() {
        panic!("product name should not be longer then 300 bytes")
    }

    s.to_string()
}

fn clean_quantity(n: u16) -> u16 {
    if n == 0 {
        panic!("quantity should be greater than 0")
    }

    n
}

fn clean_unit_price(p: u16) -> u16 {
    if p == 0 {
        panic!("unit price should be greater than 0");
    }

    p
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u16) -> Self {
        let product_name = clean_product_name(product_name);
        let quantity = clean_quantity(quantity);
        let unit_price = clean_unit_price(unit_price);

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        let product_name = clean_product_name(product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u16) {
        let quantity = clean_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u16) {
        let unit_price = clean_unit_price(unit_price);
        self.unit_price = unit_price;
    }

    pub fn total(&self) -> u16 {
        self.unit_price * self.quantity
    }
}
