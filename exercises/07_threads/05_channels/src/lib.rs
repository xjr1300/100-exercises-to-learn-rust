use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

use data::TicketDraft;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
// サーバースレッドを生み出すことによって、システムを開始します。
// それは、サーバーと相互作用するために、1つ以上のクライアントによって使用される`Sender`インスタンスを返します。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
// サーバーのタスクは**決して**停止してはなりません。
// チャネルに現れるコマンドを待ち、それを実行した後、次のコマンドを待つことを開始するためにループします。
pub fn server(receiver: Receiver<Command>) {
    loop {
        let _draft = receiver
            .recv()
            .expect("Did you actually spawn a thread? The channel is closed!");
    }
}
