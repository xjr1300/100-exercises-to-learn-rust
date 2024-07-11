// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
//  than `impl Trait` syntax.
// `impl Trait`構文ではなくジェネリックパラメーターを使用して`TicketStore::add_ticket`のシグネチャーを再加工してください。

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // Using `Into<Ticket>` as the type parameter for `ticket` allows the method to accept any type
    // that can be infallibly converted into a `Ticket`.
    // This can make it nicer to use the method, as it removes the syntax noise of `.into()`
    // from the calling site. It can worsen the quality of the compiler error messages, though.
    // `ticket`に対して型パラメーターとして`Into<Ticket>`を使用することは、メソッドが失敗することなく`Ticket`に変換できる
    // 任意の型を受け付けるようにします。
    // これは、呼び出し側から`.into()`の構文のノイズを除去するため、メソッドを使用しやすくします。
    // ただし、コンパイラーのエラーメッセージの品質が悪くなる可能性があります。
    pub fn add_ticket<T>(&mut self, ticket: T)
    where
        T: Into<Ticket>,
    {
        self.tickets.push(ticket.into());
    }
    /*
    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        // `add_ticket`が引数の位置に`impl Trait`構文を使用している場合、これはコンパイルされません。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
