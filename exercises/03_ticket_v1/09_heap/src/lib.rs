pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
// このセクションで学んだことに基づいて、`todo!()`をそれぞれの型の正しい**スタックサイズ**に置き換えてください。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 8 * 3);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        // これは難しい問題です！
        // 今回、「直感的な」答えは正しい答えになりますが、一般的に、構造体のメモリレイアウトはより複雑なトピックです。
        // 興味がある場合、より詳細はRustonomiconの「データレイアウト」セクションを確認してください。
        assert_eq!(size_of::<Ticket>(), 8 * 3 * 3);
    }
}
