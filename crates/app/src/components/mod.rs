mod challenges_view;
mod code_display;
mod hidden_input;
mod highlight;
mod history_view;
mod project_view;
mod results_summary;
mod snippet_importer;
mod snippet_picker;
mod stats_bar;
mod typing_session;
mod virtual_keyboard;

pub use challenges_view::ChallengesView;
pub use history_view::HistoryView;
pub use project_view::ProjectView;
pub use snippet_importer::SnippetImporter;
pub use snippet_picker::SnippetPicker;
pub use typing_session::TypingSession;

pub(crate) use highlight::{HighlightData, get_highlight};
