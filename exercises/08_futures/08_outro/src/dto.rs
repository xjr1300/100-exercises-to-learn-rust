use crate::models::{TicketDescription, TicketStatus, TicketTitle};

/// チケットドラフト
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

/// チケットのパッチ
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<TicketStatus>,
    pub version: u64,
}
