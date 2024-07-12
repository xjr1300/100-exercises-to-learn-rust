use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub mod data;
pub mod store;

use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CommandError {
    #[error("The store is overloaded")]
    Overloaded,
    #[error("The version is not match")]
    VersionNotMatch,
    #[error("The command response does not receive")]
    ResponseDoesNotReceive,
    #[error("The ticket does not exist")]
    TicketDoesNotExist,
}

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug, Clone)]
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: SyncSender<Option<Ticket>>,
    },
    Update {
        patch: TicketPatch,
        response_sender: SyncSender<Result<(), CommandError>>,
    },
}

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

fn try_send_command(sender: &SyncSender<Command>, command: Command) -> Result<(), CommandError> {
    sender
        .try_send(command)
        .map_err(|_| CommandError::Overloaded)
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> CommandResult<TicketId> {
        let (response_sender, response_receiver) = sync_channel(1);
        let command = Command::Insert {
            draft,
            response_sender,
        };
        try_send_command(&self.sender, command)?;

        response_receiver
            .recv()
            .map_err(|_| CommandError::ResponseDoesNotReceive)
    }

    pub fn get(&self, id: TicketId) -> CommandResult<Option<Ticket>> {
        let (response_sender, response_receiver) = sync_channel(1);
        let command = Command::Get {
            id,
            response_sender,
        };
        try_send_command(&self.sender, command)?;

        response_receiver
            .recv()
            .map_err(|_| CommandError::ResponseDoesNotReceive)
    }

    pub fn update(&self, patch: TicketPatch) -> CommandResult<()> {
        let (response_sender, response_receiver) = sync_channel(1);
        let command = Command::Update {
            patch,
            response_sender,
        };
        try_send_command(&self.sender, command)?;

        response_receiver
            .recv()
            .map_err(|_| CommandError::ResponseDoesNotReceive)?
    }
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::default();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let _ = response_sender.send(store.add_ticket(draft));
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let _ = response_sender.send(store.get(id).cloned());
            }
            Ok(Command::Update {
                patch,
                response_sender,
            }) => {
                let _ = response_sender.send(store.update_ticket(patch));
            }
            Err(_) => break,
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(|| server(receiver));

    TicketStoreClient { sender }
}
