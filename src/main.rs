
mod app;
mod errors;
mod manager;
mod network;
mod ui;
mod utils;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use manager::ExtensionManager;
use network::ApiClient;

use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use utils::config_loader::ConfigLoader;
use utils::logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    logger::init_logger()?;


    let config = ConfigLoader::new().load_config()?;


    let api_client = ApiClient::new(&config.api_endpoint, config.timeout_secs)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::builder()
        .title("VSCode Extension Manager")
        .config(config)
        .api_client(api_client)
        .build()?;


    let mut manager = ExtensionManager::new(app.api_client.clone());


    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });


    let res = run_app(&mut terminal, app, &mut manager, rx).await;


    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        log::error!("Application error: {:?}", err);
    }

    Ok(())
}

enum Event {
    Input(event::KeyEvent),
    Tick,
}

async fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    manager: &mut ExtensionManager,
    rx: mpsc::Receiver<Event>,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, &app, manager))?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => {
                    let selected = app.selected();

             
                    if let Some(ext_id) = manager.get_extension_id(selected).cloned() {
                        manager.toggle_extension(&ext_id).await?;
                    }
                }
                KeyCode::Char('s') => {
                  
                    app.open_settings();
                }
                KeyCode::Char('u') => {
               
                    manager.update_extensions().await?;
                }
                KeyCode::Char('c') => {
             
                    manager.clear_cache()?;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
}
