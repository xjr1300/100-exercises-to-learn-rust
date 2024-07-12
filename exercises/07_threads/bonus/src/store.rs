use std::collections::BTreeMap;

use crate::{
    data::{Status, Ticket, TicketDraft, TicketPatch},
    CommandError, CommandResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Debug, Default)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            version: 0,
        };
        self.tickets.insert(id, ticket);

        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn update_ticket(&mut self, patch: TicketPatch) -> CommandResult<()> {
        match self.get_mut(patch.id) {
            Some(original) => {
                if original.version == patch.version {
                    if let Some(title) = patch.title {
                        original.title = title;
                    }
                    if let Some(description) = patch.description {
                        original.description = description;
                    }
                    if let Some(status) = patch.status {
                        original.status = status;
                    }
                    original.version += 1;
                    Ok(())
                } else {
                    Err(CommandError::VersionNotMatch)
                }
            }
            None => Err(CommandError::TicketDoesNotExist),
        }
    }
}
