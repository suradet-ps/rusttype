fn ext(&self) -> Option<String> {
    if self.opts.case_insensitive {
        return None;
    }
    let start = match *self.tokens.get(0)? {
        Token::RecursivePrefix => 1,
        _ => 0,
    };
    match *self.tokens.get(start)? {
        Token::ZeroOrMore => {
            // If there was no recursive prefix, then we only permit
            // `*` if `*` can match a `/`. For example, if `*` can't
            // match `/`, then `*.c` doesn't match `foo/bar.c`.
            if start == 0 && self.opts.literal_separator {
                return None;
            }
        }
        _ => return None,
    }
    match *self.tokens.get(start + 1)? {
        Token::Literal('.') => {}
        _ => return None,
    }
    let mut lit = ".".to_string();
    for t in self.tokens[start + 2..].iter() {
        match *t {
            Token::Literal('.') | Token::Literal('/') => return None,
            Token::Literal(c) => lit.push(c),
            _ => return None,
        }
    }
    if lit.is_empty() { None } else { Some(lit) }
}
