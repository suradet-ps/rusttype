mod code_display;
mod hidden_input;
mod highlight;
mod history_view;
mod results_summary;
mod snippet_importer;
mod snippet_picker;
mod stats_bar;
mod typing_session;
mod virtual_keyboard;

pub use history_view::HistoryView;
pub use snippet_importer::SnippetImporter;
pub use snippet_picker::SnippetPicker;
pub use typing_session::TypingSession;

pub(crate) use highlight::{HighlightData, get_highlight};
