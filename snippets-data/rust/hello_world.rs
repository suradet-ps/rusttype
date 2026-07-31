fn main() {
    let name = "World";
    let greeting = format!("Hello, {}!", name);
    println!("{greeting}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let name = "Rust";
        let result = format!("Hello, {}!", name);
        assert!(result.contains(name));
    }
}
