// TODO: Implement `Debug`, `Display` and `Error` for the `TicketNewError` enum.
//  When implementing `Display`, you may want to use the `write!` macro from Rust's standard library.
//  The docs for the `std::fmt` module are a good place to start and look for examples:
//  https://doc.rust-lang.org/std/fmt/index.html#write
// `Debug`、`Display`そして`Error`を`TicketNewError`列挙型に実装してください。
// `Display`を実装するとき、Rust標準ライブラリの`write!`マクロを使用したいかもしれません。
// `std::fmt`モジュールのドキュメントは、開始そして例を確認する良い場所です。

use std::fmt;

#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            Self::TitleError(msg) => msg,
            Self::DescriptionError(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for TicketNewError {}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
// `easy_ticket`は、タイトルが無効なとき、`TicketNewError`列挙型の関連するバリアント内に保存されたメッセージを使用してパニックします。
// 説明が無効な場合、代わりに、「Description not provided」というデフォルトの説明を使用します。
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::TitleError(message)) => panic!("{message}"),
        Err(TicketNewError::DescriptionError(_)) => {
            Ticket::new(title, String::from("Description not provided"), status).unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 bytes".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 bytes".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    use static_assertions::assert_impl_one;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    fn display_is_correctly_implemented() {
        let ticket = Ticket::new("".into(), valid_description(), Status::ToDo);
        assert_eq!(format!("{}", ticket.unwrap_err()), "Title cannot be empty");
    }

    assert_impl_one!(TicketNewError: std::error::Error);
}
