// TODO: you have something to do in each of the modules in this crate!
// 次のクレートのそれぞれのモジュールですることが何かあります。
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
// Rustにおける一般的なパターンは、複数の（プライベートな）モジュールに分割して、
// クレートのルートでそれらのモジュールの公開部分を最エクスポートすることです。
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
// これは、クレートの内部構造をユーザーから隠し、コードを好きなように整理できるようにします。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
// フィールドをプライベートにする必要はありません！
// それぞれのフィールドは、それ独自の検証ロジックをカプセル化しているため、構造体の不変条件を破る方法で、
// ユーザーが`Ticket`のフィールドを修正するリスクはありません。
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
// よく考えてください: 複数フィールドにまたがる不変条件を持っている場合、それらの不変条件がまだ維持されている
// ことを確実にして、フィールドをプライベートにする必要があります。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
