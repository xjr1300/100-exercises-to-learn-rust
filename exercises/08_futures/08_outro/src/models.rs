/// チケットID
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct TicketId(pub u64);

/// チケットタイトル
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TicketTitle(pub String);

/// チケットタイトルエラー
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum TicketTitleError {
    #[error("チケットのタイトルは空にできません。")]
    Empty,
    #[error("チケットのタイトルは50文字以下です。")]
    TooLong,
}

/// チケットタイトルの最大文字数
const TICKET_TITLE_MAX_CHARS: usize = 50;

/// 文字列からチケットのタイトルを構築する。
///
/// # 引数
///
/// * `s` - チケットのタイトルを表現する文字列
///
/// # 戻り値
///
/// チケットのタイトル
///
fn ticket_title_from_str(s: &str) -> Result<TicketTitle, TicketTitleError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(TicketTitleError::Empty);
    }
    if TICKET_TITLE_MAX_CHARS < s.len() {
        return Err(TicketTitleError::TooLong);
    }

    Ok(TicketTitle(s.into()))
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ticket_title_from_str(&value)
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ticket_title_from_str(value)
    }
}

/// チケットの説明
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TicketDescription(pub String);

/// チケットの説明の最大文字数
pub const TICKET_DESCRIPTION_MAX_CHARS: usize = 500;

/// チケット説明エラー
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum TicketDescriptionError {
    #[error("チケットの説明を空にできません。")]
    Empty,
    #[error("チケットの説明は500文字以内です。")]
    TooLong,
}

/// 文字列からチケットの説明を構築する。
///
/// # 引数
///
/// * `s` - チケットの説明を表現する文字列
///
/// # 戻り値
///
/// チケットの説明
fn ticket_description_from_str(s: &str) -> Result<TicketDescription, TicketDescriptionError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(TicketDescriptionError::Empty);
    }
    if TICKET_DESCRIPTION_MAX_CHARS < s.len() {
        return Err(TicketDescriptionError::TooLong);
    }

    Ok(TicketDescription(s.into()))
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ticket_description_from_str(&value)
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ticket_description_from_str(value)
    }
}

/// チケットステータス
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TicketStatus {
    /// 未着手
    ToDo,
    /// 進行中
    InProgress,
    /// 完了
    Done,
}

/// チケットステータスエラー
#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error(r#"チケットのステータスは、`"ToDo"`、`"InProgress"`または`"Done"`のいずれかです。"#)]
pub struct TicketStatusError;

/// 文字列からチケットのステータスを構築する。
///
/// # 引数
///
/// * `s` - チケットのステータスを表現する文字列
///
/// # 戻り値
///
/// チケットのステータス
fn ticket_status_from_str(s: &str) -> Result<TicketStatus, TicketStatusError> {
    match s.trim().to_lowercase().as_str() {
        "todo" => Ok(TicketStatus::ToDo),
        "inprogress" => Ok(TicketStatus::InProgress),
        "done" => Ok(TicketStatus::Done),
        _ => Err(TicketStatusError),
    }
}

impl TryFrom<String> for TicketStatus {
    type Error = TicketStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ticket_status_from_str(&value)
    }
}

impl TryFrom<&str> for TicketStatus {
    type Error = TicketStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ticket_status_from_str(value)
    }
}

/// チケット
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
    pub version: u64,
}

impl Ticket {
    pub fn new(id: TicketId, title: TicketTitle, description: TicketDescription) -> Self {
        Self {
            id,
            title,
            description,
            status: TicketStatus::ToDo,
            version: 0,
        }
    }
}
