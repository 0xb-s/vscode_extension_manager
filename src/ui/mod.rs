pub mod components;
pub mod dialogs;
pub mod events;
pub mod styles;

use crate::app::App;
use crate::manager::ExtensionManager;
use components::{draw_main, draw_sidebar};
use dialogs::draw_dialogs;
use styles::apply_theme;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub fn draw<B: tui::backend::Backend>(f: &mut Frame<B>, app: &App, manager: &ExtensionManager) {
    apply_theme(f, &app.config.theme);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    draw_sidebar(f, app, chunks[0]);
    draw_main(f, manager, chunks[1]);


    draw_dialogs(f, app);
}
