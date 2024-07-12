use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
// クライアントの実装を具体化してください。
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    // 簡素化のために、すべてのエラーに対して自由にパニックしてください。
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (response_sender, response_receiver) = std::sync::mpsc::channel::<TicketId>();
        let command = Command::Insert {
            draft,
            response_channel: response_sender,
        };
        self.sender
            .send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");

        response_receiver
            .recv()
            .expect("Did you actually spawn a thread? The channel is closed!")
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        let command = Command::Get {
            id,
            response_channel: response_sender,
        };
        self.sender
            .send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");

        response_receiver
            .recv()
            .expect("Did you actually spawn a thread? The channel is closed!")
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));

    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
// もはやパニックしません！現在、これはライブラリの内部的な詳細になります。
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                // 送信者が存在しないため、安全に閉じて、サーバーをシャットダウンします。
                break;
            }
        }
    }
}
