use status::ParseStatusError;

use crate::status::Status;

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
// 前の演習の1つでモジュールを宣言する方法を確認しましたが、別のファイルにそれらを抽出する方法を確認していません。
// 今回、それを修正しましょう。
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
// 最も単純な場合では、抽出されたモジュールが1つのファイルの場合、モジュールと同じ名前の新しいファイルを作成することで十分で、
// そこにモジュールの内容を移動します。
// モジュールファイルはモジュールを宣言するファイルとして同じディレクトリ内に配置されます。
// `src/lib.rs`の場合、status.rs`は`src`ディレクトリ内に配置されます。
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.
// 状態を示す文字列が無効なときのために、`TicketNewError`の新しいエラーバリアントを追加してください。
// そのバリアントのエラーで`source`が呼ばれたとき、`None`ではなく`ParseStatusError`を返してください。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{inner}")]
    InvalidStatus {
        #[from]
        inner: status::ParseStatusError,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.
        let status: Status = status
            .try_into()
            .map_err(|e: ParseStatusError| TicketNewError::from(e))?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
