// TODO: A (derivable) trait implementation is missing for this exercise to compile successfully.
//   Fix it!
// 成功裏にコンパイルするために、この演習の導出可能なトレイトの実装は欠落しています。
// それを修正してください。
//
// # `Debug` primer（`Debug`の入門）
//
// `Debug` returns a representation of a Rust type that's suitable for debugging (hence the name).
// `assert_eq!` requires `Ticket` to implement `Debug` because, when the assertion fails, it tries to
// print both sides of the comparison to the terminal.
// If the compared type doesn't implement `Debug`, it doesn't know how to represent them!
// `Debug`は、デバッグするために適したRust型の表現を返します（それが名前の由来）。
// `assert_eq!`は、アサーションが失敗したときに、比較の両側をターミナルに出力することを試みるため、`Debug`を実装することを`Ticket`に要求します。
// 比較される型が`Debug`を実装していない場合、コンパイラーはそれらを表現する方法を知りません！

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }
}
