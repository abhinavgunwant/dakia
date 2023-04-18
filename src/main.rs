use std::{ io, thread, time::Duration, error::Error };

use tui::{
    backend::{ Backend, CrosstermBackend },
    widgets::{ BorderType, Block, Borders },
    layout::{ Alignment, Layout, Constraint, Direction },
    style::{ Color, Modifier, Style },
    text::Span,
    Frame, Terminal,
};

use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen
    },
};

use ui::{
    ui_func,
    state::{ UiState, InputMode, EditorMode, UIElement, Method },
    widgets::{ label::Label, text_input::TextInput },
};

mod ui;
mod user_input;

fn main() -> Result<(), io::Error> {
    let mut input: String = String::new();
    let mut uistate = UiState::default();

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
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut uistate: UiState) -> io::Result<()> {
    loop {
        let cloned_state = uistate.clone();
        terminal.draw(|f| ui_func(f, cloned_state))?;

        if uistate.clone().editor_mode() == EditorMode::Normal
            || uistate.clone().input_mode() == InputMode::Normal
        {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => { return Ok(()); },
                    KeyCode::Char(c) => {
                        match uistate.active_element() {
                            UIElement::URL => { uistate.append_url(c); },
                            UIElement::Method => {
                                match c.to_digit(10) {
                                    Some(num) => {
                                        if num > 0 {
                                            uistate.set_method(
                                                Method::from_val((num - 1) as u8)
                                            );
                                        }
                                    },

                                    None => {
                                        let c_ = c.to_ascii_uppercase();
                                        let allowed_chars = "GPUDH";

                                        if allowed_chars.contains(
                                                c_.to_string().as_str()
                                            )
                                        {
                                            match c_ {
                                                'G' => {
                                                    uistate.set_method(
                                                        Method::GET
                                                    );
                                                },
                                                'P' => {
                                                    uistate.set_method(
                                                        Method::POST
                                                    );
                                                },
                                                'U' => {
                                                    uistate.set_method(
                                                        Method::PUT
                                                    );
                                                },
                                                'D' => {
                                                    uistate.set_method(
                                                        Method::DELETE
                                                    );
                                                },
                                                'H' => {
                                                    uistate.set_method(
                                                        Method::HEADER
                                                    );
                                                },
                                                _ => {},
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {},
                        }
                    },
                    KeyCode::Backspace => { uistate.pop_url(); },
                    KeyCode::Enter => { uistate.url().push('\n'); },
                    KeyCode::Tab => { uistate.activate_next_element(); },
                    KeyCode::BackTab => { uistate.activate_previous_element(); },
                    _ => {  },
                };
            }
        }
    }
}

