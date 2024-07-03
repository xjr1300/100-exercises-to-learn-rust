struct Ticket {
    title: String,
    description: String,
    status: String,
}

/// タイトルを検証する。
fn validate_title(title: &str) {
    let title = title.trim();
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if 50 < title.as_bytes().len() {
        panic!("Title cannot be longer than 50 bytes")
    }
}

/// 説明を検証する。
fn validate_description(description: &str) {
    let description = description.trim();
    if description.is_empty() {
        panic!("Description cannot be empty")
    }
    if 500 < description.as_bytes().len() {
        panic!("Description cannot be longer than 500 bytes");
    }
}

/// 状態を検証する。
fn validate_status(status: &str) {
    if status == "To-Do" || status == "In Progress" || status == "Done" {
        return;
    }

    panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    //
    // `new`関数を実装してください。
    // 次の要求事項を満たす必要があります。
    // - `To-Do`、`In Progress`そして`Done`状態のみが許可されます。
    // - `title`と`description`フィールドは、空であってはなりません。
    // - `title`は、最大50バイトまででなくてはなりません。
    // 要求事項を満たさない場合はパニックしてください。
    //
    // `String`メソッドと同様に、前の演習で学んだことをしようしなければなりません。
    // 最も適切な選択肢を見つけるために、Rust標準ライブラリのドキュメントを使用してください。
    fn new(title: String, description: String, status: String) -> Self {
        validate_title(&title);
        validate_status(&status);
        validate_description(&description);

        Self {
            title: title.trim().to_string(),
            description: description.trim().to_string(),
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
