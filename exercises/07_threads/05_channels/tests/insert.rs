// TODO: Set `move_forward` to `true` in `ready` when you think you're done with this exercise.
//  Feel free to call an instructor to verify your solution!
// この演習を完了したいときに、`ready`内の`move_forward`を`true`に設定してください。
// 解答を検証するために、自由にインストラクターを呼び出してください。
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // If the thread is no longer running, this will panic
        // because the channel will be closed.
        // スレッドが実行していない場合、チェネルが閉じられるため、次はパニックします。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // There's very little that we can check automatically in this exercise,
    // since our server doesn't expose any **read** actions.
    // We have no way to know if the inserts are actually happening and if they
    // are happening correctly.
    // サーバーは**読み込み**動作を公開していないため、この演習で自動的に検証できるものはありません。
    // 挿入が実際に発生しているか、またそれらが正確に発生しているかを知る方法がありません。
    let move_forward = true;

    assert!(move_forward);
}
