pub fn escape(bytes: &[u8]) -> String {
    bytes.escape_bytes().to_string()
}

/// Escapes an OS string into a human readable string.
///
/// This is like [`escape`], but accepts an OS string.
pub fn escape_os(string: &OsStr) -> String {
    escape(Vec::from_os_str_lossy(string).as_bytes())
}
