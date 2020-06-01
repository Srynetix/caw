// State

#[derive(Clone)]
pub struct UiState {
    pub show_window: bool,
    pub show_help: bool,
    pub show_about: bool,
    pub cursor_size: usize,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            show_window: true,
            show_help: true,
            show_about: false,
            cursor_size: 10,
        }
    }
}
