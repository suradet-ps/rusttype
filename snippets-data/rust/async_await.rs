async fn fetch_data(url: &str) -> Result<String, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".into());
    }
    Ok(format!("Response from {url}"))
}

async fn process_urls(urls: &[&str]) -> Vec<Result<String, String>> {
    let mut results = Vec::new();
    for url in urls {
        results.push(fetch_data(url).await);
    }
    results
}

struct AsyncCounter {
    count: u32,
}

impl AsyncCounter {
    fn new() -> Self {
        Self { count: 0 }
    }

    async fn increment(&mut self) -> u32 {
        self.count += 1;
        self.count
    }

    fn get(&self) -> u32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_data_empty_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(fetch_data(""));
        assert!(result.is_err());
    }

    #[test]
    fn fetch_data_valid_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(fetch_data("https://example.com"));
        assert!(result.is_ok());
        assert!(result.unwrap().contains("example.com"));
    }

    #[test]
    fn counter_increments() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut counter = AsyncCounter::new();
        assert_eq!(rt.block_on(counter.increment()), 1);
        assert_eq!(rt.block_on(counter.increment()), 2);
    }
}
