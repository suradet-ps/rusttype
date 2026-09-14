fn literal(&self) -> Option<String> {
    if self.opts.case_insensitive {
        return None;
    }
    let mut lit = String::new();
    for t in &*self.tokens {
        let Token::Literal(c) = *t else { return None };
        lit.push(c);
    }
    if lit.is_empty() { None } else { Some(lit) }
}
