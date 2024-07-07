// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.
// コードがコンパイルできるように、必要な`Clone`の実装と呼び出しを追加してください。

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let cloned = ticket.clone();
    (ticket, cloned.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
