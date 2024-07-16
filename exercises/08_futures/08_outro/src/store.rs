use std::collections::BTreeMap;

use crate::dto::{TicketDraft, TicketPatch};
use crate::models::{Ticket, TicketId};

/// チケットストア
#[derive(Debug, Default)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    next_id: u64,
}

impl TicketStore {
    /// チケットを追加する。
    ///
    /// # 引数
    ///
    /// * `draft` - 追加するチケットのドラフト
    ///
    /// # 戻り値
    ///
    /// 追加したチケットのID
    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.next_id);
        let ticket = Ticket::new(id, draft.title, draft.description);
        self.next_id += 1;
        self.tickets.insert(id, ticket);

        id
    }

    /// チケットIDを指定して、チケットの参照を取得する。
    ///
    /// # 引数
    ///
    /// * `id` - チケットID
    ///
    /// # 戻り値
    ///
    /// チケットの参照
    pub fn get(&self, id: TicketId) -> TicketStoreResult<&Ticket> {
        self.tickets.get(&id).ok_or(TicketStoreError::NotFound)
    }

    /// チケットIDを指定して、チケットの可変参照を取得する。
    ///
    /// # 引数
    ///
    /// * `id` - チケットID
    ///
    /// # 戻り値
    ///
    /// チケットの可変参照
    pub fn get_mut(&mut self, id: TicketId) -> TicketStoreResult<&mut Ticket> {
        self.tickets.get_mut(&id).ok_or(TicketStoreError::NotFound)
    }

    /// チケットを更新する。
    ///
    /// # 引数
    ///
    /// * `id` - 更新するチケットのチケットID
    /// * `patch` - チケットのパッチ
    ///
    /// # 戻り値
    ///
    /// `()`
    pub fn update_ticket(&mut self, id: TicketId, patch: TicketPatch) -> TicketStoreResult<()> {
        let target = self.get_mut(id)?;
        if patch.version != target.version {
            return Err(TicketStoreError::VersionNotMatch);
        }
        if let Some(title) = patch.title {
            target.title = title;
        }
        if let Some(description) = patch.description {
            target.description = description;
        }
        if let Some(status) = patch.status {
            target.status = status;
        }
        target.version += 1;

        Ok(())
    }
}

/// チケットストアエラー
#[derive(Debug, Clone, thiserror::Error)]
pub enum TicketStoreError {
    #[error("チケットが見つかりません。")]
    NotFound,
    #[error("チケットのバージョンが一致しません。")]
    VersionNotMatch,
}

/// チケットストア結果
type TicketStoreResult<T> = Result<T, TicketStoreError>;
