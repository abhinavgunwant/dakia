mod ui;
mod user_input;
mod api;

use std::io;

use log::{ info, error };
use log4rs;
use tui::{
    backend::{ Backend, CrosstermBackend },
    Terminal,
};

use crossterm::{
    event::{ DisableMouseCapture, EnableMouseCapture },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen
    },
};

use ui::{ ui_func, state::UiState };

use user_input::process_user_input;

const VERSION: &str = "v0.0.1";

fn main() -> Result<(), io::Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let uistate = UiState::default();

    info!("dakia {}. Starting up...", VERSION);

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, uistate);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        error!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut uistate: UiState)
    -> io::Result<()> {
    loop {
        terminal.draw(|f| ui_func(f, &mut uistate))?;

        match process_user_input(&mut uistate) {
            Ok(exit) => {
                if exit {
                    return Ok(());
                }
            },

            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

