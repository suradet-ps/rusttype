fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct TextHolder<'a> {
    text: &'a str,
    label: &'a str,
}

impl<'a> TextHolder<'a> {
    fn new(text: &'a str, label: &'a str) -> Self {
        Self { text, label }
    }

    fn formatted(&self) -> String {
        format!("[{}]: {}", self.label, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_simple() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn first_word_no_space() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn longest_returns_longer() {
        assert_eq!(longest("ab", "xyz"), "xyz");
        assert_eq!(longest("abc", "xy"), "abc");
    }

    #[test]
    fn longest_equal_length() {
        let result = longest("abc", "xyz");
        assert!(result == "abc" || result == "xyz");
    }

    #[test]
    fn text_holder_formatted() {
        let holder = TextHolder::new("content", "title");
        assert_eq!(holder.formatted(), "[title]: content");
    }
}
