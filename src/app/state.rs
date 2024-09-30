use super::config::ViewMode;
#[derive(PartialEq, Debug, Eq, Clone)]
pub enum View {
    InstalledExtensions,
    AvailableExtensions,
    Settings,
    SearchResults,
}

pub struct AppState {
    pub selected: usize,
    pub current_view: View,
    pub view_mode: ViewMode,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            current_view: View::InstalledExtensions,
            view_mode: ViewMode::List,
        }
    }

    pub fn next(&mut self) {
        self.selected += 1;
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
        self.selected = 0;
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::List => ViewMode::Grid,
            ViewMode::Grid => ViewMode::List,
        };
    }
}
