// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut str` work? Why or why not?
// 文字列内のすべての文字を小文字に変換して、入力をその場で修正する、`lowercase`と名付けられた関数を定義してください。
// `&mut String`を受け取る必要があるでしょうか？`&mut str`は機能するでしょうか？なぜ機能して、なぜ機能しないのでしょうか？
fn lowercase(s: &mut str) {
    s.make_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
