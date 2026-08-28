//! QWERTY keyboard layout data for the on-screen practice keyboard.
//!
//! Pure data + pure mapping - no browser APIs, so this module is unit-testable
//! without WASM. The layout is a physical-keyboard concern (QWERTY), not a
//! language model, so RustType stays Rust-only.

/// The eight fingers used in the touch-typing method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finger {
  LeftPinky,
  LeftRing,
  LeftMiddle,
  LeftIndex,
  RightIndex,
  RightMiddle,
  RightRing,
  RightPinky,
}

impl Finger {
  /// CSS class giving this finger its colour on the virtual keyboard.
  pub fn class(self) -> &'static str {
    match self {
      Self::LeftPinky => "finger-lp",
      Self::LeftRing => "finger-lr",
      Self::LeftMiddle => "finger-lm",
      Self::LeftIndex => "finger-li",
      Self::RightIndex => "finger-ri",
      Self::RightMiddle => "finger-rm",
      Self::RightRing => "finger-rr",
      Self::RightPinky => "finger-rp",
    }
  }

  /// Human-readable finger name for the legend.
  pub fn name(self) -> &'static str {
    match self {
      Self::LeftPinky => "Left pinky",
      Self::LeftRing => "Left ring",
      Self::LeftMiddle => "Left middle",
      Self::LeftIndex => "Left index",
      Self::RightIndex => "Right index",
      Self::RightMiddle => "Right middle",
      Self::RightRing => "Right ring",
      Self::RightPinky => "Right pinky",
    }
  }

  /// All fingers, in legend display order.
  pub const ALL: [Finger; 8] = [
    Finger::LeftPinky,
    Finger::LeftRing,
    Finger::LeftMiddle,
    Finger::LeftIndex,
    Finger::RightIndex,
    Finger::RightMiddle,
    Finger::RightRing,
    Finger::RightPinky,
  ];
}

/// A single printable key on the QWERTY layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyDef {
  /// Unshifted label, e.g. `"["`.
  pub base: &'static str,
  /// Shifted label, e.g. `"{"`; `None` when the key has no shifted symbol.
  pub shift: Option<&'static str>,
  /// Finger that presses this key.
  pub finger: Finger,
}

impl KeyDef {
  const fn new(base: &'static str, shift: Option<&'static str>, finger: Finger) -> Self {
    Self {
      base,
      shift,
      finger,
    }
  }
}

/// Number row: `` ` `` `1` … `0` `-` `=`.
pub const NUMBER_ROW: [KeyDef; 13] = [
  KeyDef::new("`", Some("~"), Finger::LeftPinky),
  KeyDef::new("1", Some("!"), Finger::LeftPinky),
  KeyDef::new("2", Some("@"), Finger::LeftRing),
  KeyDef::new("3", Some("#"), Finger::LeftMiddle),
  KeyDef::new("4", Some("$"), Finger::LeftIndex),
  KeyDef::new("5", Some("%"), Finger::LeftIndex),
  KeyDef::new("6", Some("^"), Finger::RightIndex),
  KeyDef::new("7", Some("&"), Finger::RightIndex),
  KeyDef::new("8", Some("*"), Finger::RightMiddle),
  KeyDef::new("9", Some("("), Finger::RightRing),
  KeyDef::new("0", Some(")"), Finger::RightPinky),
  KeyDef::new("-", Some("_"), Finger::RightPinky),
  KeyDef::new("=", Some("+"), Finger::RightPinky),
];

/// Top letter row: `q` … `p` `[` `]` `\`.
pub const TOP_ROW: [KeyDef; 13] = [
  KeyDef::new("q", None, Finger::LeftPinky),
  KeyDef::new("w", None, Finger::LeftRing),
  KeyDef::new("e", None, Finger::LeftMiddle),
  KeyDef::new("r", None, Finger::LeftIndex),
  KeyDef::new("t", None, Finger::LeftIndex),
  KeyDef::new("y", None, Finger::RightIndex),
  KeyDef::new("u", None, Finger::RightIndex),
  KeyDef::new("i", None, Finger::RightMiddle),
  KeyDef::new("o", None, Finger::RightRing),
  KeyDef::new("p", None, Finger::RightPinky),
  KeyDef::new("[", Some("{"), Finger::RightPinky),
  KeyDef::new("]", Some("}"), Finger::RightPinky),
  KeyDef::new("\\", Some("|"), Finger::RightPinky),
];

/// Home row: `a` … `l` `;` `'`.
pub const HOME_ROW: [KeyDef; 11] = [
  KeyDef::new("a", None, Finger::LeftPinky),
  KeyDef::new("s", None, Finger::LeftRing),
  KeyDef::new("d", None, Finger::LeftMiddle),
  KeyDef::new("f", None, Finger::LeftIndex),
  KeyDef::new("g", None, Finger::LeftIndex),
  KeyDef::new("h", None, Finger::RightIndex),
  KeyDef::new("j", None, Finger::RightIndex),
  KeyDef::new("k", None, Finger::RightMiddle),
  KeyDef::new("l", None, Finger::RightRing),
  KeyDef::new(";", Some(":"), Finger::RightPinky),
  KeyDef::new("'", Some("\""), Finger::RightPinky),
];

/// Bottom letter row: `z` … `/`.
pub const BOTTOM_ROW: [KeyDef; 10] = [
  KeyDef::new("z", None, Finger::LeftPinky),
  KeyDef::new("x", None, Finger::LeftRing),
  KeyDef::new("c", None, Finger::LeftMiddle),
  KeyDef::new("v", None, Finger::LeftIndex),
  KeyDef::new("b", None, Finger::LeftIndex),
  KeyDef::new("n", None, Finger::RightIndex),
  KeyDef::new("m", None, Finger::RightIndex),
  KeyDef::new(",", Some("<"), Finger::RightMiddle),
  KeyDef::new(".", Some(">"), Finger::RightRing),
  KeyDef::new("/", Some("?"), Finger::RightPinky),
];

/// A highlightable key on the virtual keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyId {
  /// A printable key at row/column of the QWERTY key rows.
  Char {
    row: usize,
    col: usize,
  },
  Tab,
  Enter,
  Space,
  /// Either shift key - highlighted whenever the next character needs Shift.
  Shift,
}

/// Whether producing `c` on a US QWERTY layout requires holding Shift.
///
/// Covers uppercase letters and every shifted symbol on the key rows.
pub fn needs_shift(c: char) -> bool {
  c.is_ascii_uppercase()
    || matches!(
      c,
      '~'
        | '!'
        | '@'
        | '#'
        | '$'
        | '%'
        | '^'
        | '&'
        | '*'
        | '('
        | ')'
        | '_'
        | '+'
        | '{'
        | '}'
        | '|'
        | ':'
        | '"'
        | '<'
        | '>'
        | '?'
    )
}

/// Map a character to the physical key that produces it.
///
/// Letters, digits, and symbols map to their QWERTY key (shifted symbols map
/// to the same key as their unshifted base). Whitespace and control
/// characters map to `Tab` / `Enter` / `Space`. Unproducible characters map
/// to `None`.
pub fn key_id_for_char(c: char) -> Option<KeyId> {
  match c {
    '\n' => Some(KeyId::Enter),
    '\t' => Some(KeyId::Tab),
    ' ' => Some(KeyId::Space),
    'a'..='z' => letter_key(c),
    'A'..='Z' => letter_key(c.to_ascii_lowercase()),
    '0'..='9' => Some(KeyId::Char {
      row: 0,
      col: (c as usize - '0' as usize + 9) % 10 + 1,
    }),
    '`' | '~' => Some(KeyId::Char { row: 0, col: 0 }),
    '!' => Some(KeyId::Char { row: 0, col: 1 }),
    '@' => Some(KeyId::Char { row: 0, col: 2 }),
    '#' => Some(KeyId::Char { row: 0, col: 3 }),
    '$' => Some(KeyId::Char { row: 0, col: 4 }),
    '%' => Some(KeyId::Char { row: 0, col: 5 }),
    '^' => Some(KeyId::Char { row: 0, col: 6 }),
    '&' => Some(KeyId::Char { row: 0, col: 7 }),
    '*' => Some(KeyId::Char { row: 0, col: 8 }),
    '(' => Some(KeyId::Char { row: 0, col: 9 }),
    ')' => Some(KeyId::Char { row: 0, col: 10 }),
    '-' | '_' => Some(KeyId::Char { row: 0, col: 11 }),
    '=' | '+' => Some(KeyId::Char { row: 0, col: 12 }),
    '[' | '{' => Some(KeyId::Char { row: 1, col: 10 }),
    ']' | '}' => Some(KeyId::Char { row: 1, col: 11 }),
    '\\' | '|' => Some(KeyId::Char { row: 1, col: 12 }),
    ';' | ':' => Some(KeyId::Char { row: 2, col: 9 }),
    '\'' | '"' => Some(KeyId::Char { row: 2, col: 10 }),
    ',' | '<' => Some(KeyId::Char { row: 3, col: 7 }),
    '.' | '>' => Some(KeyId::Char { row: 3, col: 8 }),
    '/' | '?' => Some(KeyId::Char { row: 3, col: 9 }),
    _ => None,
  }
}

/// Locate a lowercase letter within the three letter rows.
fn letter_key(c: char) -> Option<KeyId> {
  const ROWS: [(&str, usize); 3] = [("qwertyuiop", 1), ("asdfghjkl", 2), ("zxcvbnm", 3)];
  for (letters, row) in ROWS {
    if let Some(col) = letters.find(c) {
      return Some(KeyId::Char { row, col });
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn letters_map_to_their_own_keys() {
    assert_eq!(key_id_for_char('a'), Some(KeyId::Char { row: 2, col: 0 }));
    assert_eq!(key_id_for_char('f'), Some(KeyId::Char { row: 2, col: 3 }));
    assert_eq!(key_id_for_char('j'), Some(KeyId::Char { row: 2, col: 6 }));
    assert_eq!(key_id_for_char('p'), Some(KeyId::Char { row: 1, col: 9 }));
    assert_eq!(key_id_for_char('z'), Some(KeyId::Char { row: 3, col: 0 }));
    assert_eq!(key_id_for_char('m'), Some(KeyId::Char { row: 3, col: 6 }));
  }

  #[test]
  fn uppercase_maps_to_the_same_key_as_lowercase() {
    assert_eq!(key_id_for_char('A'), key_id_for_char('a'));
    assert_eq!(key_id_for_char('Z'), key_id_for_char('z'));
  }

  #[test]
  fn digits_map_to_the_number_row() {
    assert_eq!(key_id_for_char('1'), Some(KeyId::Char { row: 0, col: 1 }));
    assert_eq!(key_id_for_char('0'), Some(KeyId::Char { row: 0, col: 10 }));
  }

  #[test]
  fn symbols_map_to_their_shifted_keys() {
    assert_eq!(key_id_for_char('{'), Some(KeyId::Char { row: 1, col: 10 }));
    assert_eq!(key_id_for_char('}'), Some(KeyId::Char { row: 1, col: 11 }));
    assert_eq!(key_id_for_char('|'), Some(KeyId::Char { row: 1, col: 12 }));
    assert_eq!(key_id_for_char(':'), Some(KeyId::Char { row: 2, col: 9 }));
    assert_eq!(key_id_for_char('"'), Some(KeyId::Char { row: 2, col: 10 }));
    assert_eq!(key_id_for_char('<'), Some(KeyId::Char { row: 3, col: 7 }));
    assert_eq!(key_id_for_char('>'), Some(KeyId::Char { row: 3, col: 8 }));
    assert_eq!(key_id_for_char('?'), Some(KeyId::Char { row: 3, col: 9 }));
    assert_eq!(key_id_for_char('!'), Some(KeyId::Char { row: 0, col: 1 }));
    assert_eq!(key_id_for_char('@'), Some(KeyId::Char { row: 0, col: 2 }));
    assert_eq!(key_id_for_char('^'), Some(KeyId::Char { row: 0, col: 6 }));
    assert_eq!(key_id_for_char('_'), Some(KeyId::Char { row: 0, col: 11 }));
    assert_eq!(key_id_for_char('+'), Some(KeyId::Char { row: 0, col: 12 }));
    assert_eq!(key_id_for_char('~'), Some(KeyId::Char { row: 0, col: 0 }));
  }

  #[test]
  fn every_shifted_symbol_maps_back_to_its_own_key() {
    let all_rows = [
      &NUMBER_ROW[..],
      &TOP_ROW[..],
      &HOME_ROW[..],
      &BOTTOM_ROW[..],
    ];
    for (row, keys) in all_rows.iter().enumerate() {
      for (col, key) in keys.iter().enumerate() {
        if let Some(symbol) = key.shift {
          for ch in symbol.chars() {
            assert_eq!(
              key_id_for_char(ch),
              Some(KeyId::Char { row, col }),
              "shifted symbol {ch:?} should map to row {row} col {col}"
            );
          }
        }
      }
    }
  }

  #[test]
  fn whitespace_and_control_map_to_special_keys() {
    assert_eq!(key_id_for_char('\n'), Some(KeyId::Enter));
    assert_eq!(key_id_for_char('\t'), Some(KeyId::Tab));
    assert_eq!(key_id_for_char(' '), Some(KeyId::Space));
  }

  #[test]
  fn shift_detection_covers_uppercase_and_symbols() {
    assert!(needs_shift('A') && needs_shift('Z'));
    assert!(needs_shift('{') && needs_shift('~') && needs_shift('?') && needs_shift('"'));
    assert!(!needs_shift('a') && !needs_shift('1') && !needs_shift(' ') && !needs_shift('\n'));
  }

  #[test]
  fn every_shifted_symbol_is_detected() {
    let all_rows = [
      &NUMBER_ROW[..],
      &TOP_ROW[..],
      &HOME_ROW[..],
      &BOTTOM_ROW[..],
    ];
    for keys in all_rows {
      for key in keys {
        if let Some(symbol) = key.shift {
          for ch in symbol.chars() {
            assert!(needs_shift(ch), "{ch:?} should require Shift");
          }
        }
      }
    }
  }

  #[test]
  fn fingers_follow_touch_typing_assignment() {
    assert_eq!(TOP_ROW[0].finger, Finger::LeftPinky); // q
    assert_eq!(HOME_ROW[0].finger, Finger::LeftPinky); // a
    assert_eq!(BOTTOM_ROW[0].finger, Finger::LeftPinky); // z
    assert_eq!(TOP_ROW[6].finger, Finger::RightIndex); // u
    assert_eq!(TOP_ROW[7].finger, Finger::RightMiddle); // i
    assert_eq!(HOME_ROW[10].finger, Finger::RightPinky); // '
    assert_eq!(BOTTOM_ROW[9].finger, Finger::RightPinky); // /
    assert_eq!(NUMBER_ROW[0].finger, Finger::LeftPinky); // `
    assert_eq!(NUMBER_ROW[5].finger, Finger::LeftIndex); // 5
    assert_eq!(NUMBER_ROW[6].finger, Finger::RightIndex); // 6
  }

  #[test]
  fn unproducible_characters_map_to_none() {
    assert_eq!(key_id_for_char('🦀'), None);
    assert_eq!(key_id_for_char('\u{0}'), None);
  }
}
