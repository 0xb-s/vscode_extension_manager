use crate::app::App;
use crate::manager::ExtensionManager;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

pub fn draw_sidebar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let titles = ["Installed Extensions", "Available Extensions", "Settings"];
    let tabs = Tabs::new(titles.iter().cloned().map(Into::into).collect())
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.state.selected());

    f.render_widget(tabs, area);
}

pub fn draw_main<B: Backend>(f: &mut Frame<B>, manager: &ExtensionManager, area: Rect) {
    let extensions: Vec<ListItem> = manager
        .extensions
        .iter()
        .map(|ext| {
            let status = if ext.installed {
                "[Installed]"
            } else {
                "[Not Installed]"
            };
            ListItem::new(format!("{} {}", ext.id, status))
        })
        .collect();

    let list = List::new(extensions)
        .block(Block::default().borders(Borders::ALL).title("Extensions"))
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

