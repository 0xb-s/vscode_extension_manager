use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_dialogs<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Example: Settings Dialog
    if app.state.current_view == crate::app::state::View::Settings {
        let area = centered_rect(60, 20, f.size());
        let block = Block::default()
            .title("Settings")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let paragraph = Paragraph::new("Settings are currently under development.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(block, area);
        f.render_widget(paragraph, area);
    }


}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                tui::layout::Constraint::Percentage((100 - percent_y) / 2),
                tui::layout::Constraint::Percentage(percent_y),
                tui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let horizontal_layout = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage((100 - percent_x) / 2),
                tui::layout::Constraint::Percentage(percent_x),
                tui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1]);

    horizontal_layout[1]
}

