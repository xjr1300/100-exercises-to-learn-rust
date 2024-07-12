use data::TicketDraft;
use std::sync::mpsc::{Receiver, Sender};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

use data::Ticket;

// Refer to the tests to understand the expected schema.
// 期待されるスキーマを理解するためにテストを参照してください。
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));

    sender
}

// TODO: handle incoming commands as expected.
// 期待された入ってくるコマンドを処理してください。
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let ticket_id = store.add_ticket(draft);
                response_sender
                    .send(ticket_id)
                    .expect("Did you actually spawn a thread? The channel is closed!");
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = store.get(id).cloned();
                response_sender
                    .send(ticket)
                    .expect("Did you actually spawn a thread? The channel is closed!");
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                // 送信者がいないため、安全に閉じて、サーバーをシャットダウンします。
                break;
            }
        }
    }
}
