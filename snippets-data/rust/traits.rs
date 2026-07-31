trait Summary {
    fn summarize(&self) -> String;

    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..20.min(self.summarize().len())])
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
}

fn notify(item: &dyn Summary) {
    println!("Breaking news: {}", item.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_summary() {
        let article = Article {
            title: "Rust 2024".into(),
            author: "Ferris".into(),
            content: "Great news".into(),
        };
        assert!(article.summarize().contains("Rust 2024"));
    }

    #[test]
    fn tweet_summary() {
        let tweet = Tweet {
            username: "rustlang".into(),
            text: "Hello world".into(),
        };
        assert!(tweet.summarize().contains("@rustlang"));
    }

    #[test]
    fn preview_truncates() {
        let article = Article {
            title: "A very long title that goes on".into(),
            author: "Someone".into(),
            content: "Content".into(),
        };
        assert!(article.preview().ends_with("..."));
    }
}
