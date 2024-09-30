pub mod builder;
pub mod config;
pub mod state;

use crate::network::api_client::ApiClient;
use anyhow::Result;
use builder::AppBuilder;
use config::AppConfig;
use state::AppState;

pub struct App {
    pub state: AppState,
    pub title: String,
    pub config: AppConfig,
    pub api_client: ApiClient,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn next(&mut self) {
        self.state.next();
    }

    pub fn previous(&mut self) {
        self.state.previous();
    }

    pub fn selected(&self) -> usize {
        self.state.selected()
    }

    pub fn open_settings(&mut self) {
        self.state.set_view(state::View::Settings);
    }
}
