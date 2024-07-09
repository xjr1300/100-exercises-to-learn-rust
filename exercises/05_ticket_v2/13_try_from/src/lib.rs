// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.
// `Status`に対して、`TryFrom<String>`と`TryFrom<&str>`を実装してください。
// 解析は大文字小文字を無視してください。

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

fn status_try_from_str(s: &str) -> anyhow::Result<Status> {
    match s.to_lowercase().as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(anyhow::anyhow!("invalid status")),
    }
}

impl TryFrom<String> for Status {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self> {
        status_try_from_str(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        status_try_from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
