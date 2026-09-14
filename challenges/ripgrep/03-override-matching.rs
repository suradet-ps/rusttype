pub fn matched<'a, P: AsRef<Path>>(
    &'a self,
    path: P,
    is_dir: bool,
) -> Match<Glob<'a>> {
    if self.is_empty() {
        return Match::None;
    }
    let mat = self.0.matched(path, is_dir).invert();
    if mat.is_none() && self.num_whitelists() > 0 && !is_dir {
        return Match::Ignore(Glob::unmatched());
    }
    mat.map(move |giglob| Glob(GlobInner::Matched(giglob)))
}
