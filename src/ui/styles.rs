use crate::app::config::Theme;
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    Frame,
};

pub fn apply_theme<B: Backend>(f: &mut Frame<B>, theme: &Theme) {
    match theme {
        Theme::Light => {
         
        }
        Theme::Dark => {

        }
    }
}
