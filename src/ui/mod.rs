pub mod widgets;
pub mod state;

use tui::{
    backend::{ Backend, CrosstermBackend },
    widgets::{ BorderType, Block, Borders, Tabs },
    layout::{ Alignment, Layout, Constraint, Direction },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    Frame, Terminal,
};

use widgets::{ label::Label, text_input::TextInput };
use crate::ui::state::{ UiState, Method, UIElement };

pub fn ui_func<B: Backend>(f: &mut Frame<B>, mut uistate: UiState) {
    let window_size = f.size();

    // Divides window
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(2),
        ].as_ref())
        .split(window_size);

    // Divides the top portion of the window
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
        .split(outer_chunks[0]);

    let mid_pane = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner_chunks[1]);

    // The top bar (method and url)
    let top_bar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(11),
            Constraint::Min(10),
            Constraint::Length(9),
        ].as_ref())
        .split(inner_chunks[0]);

    // The tab chunks
    let tab_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(mid_pane[0]);

    let mut ti_size = f.size();
    ti_size.height = 3;

    let mut method_size = ti_size.clone();
    method_size.width = 10;

    let mut method_border_style = Style::default();

    if uistate.active_element() == &UIElement::Method {
        method_border_style = method_border_style.fg(Color::Yellow);
    }

    let method_input = TextInput::default()
        .label(String::from(" Method "))
        .borders(Borders::ALL)
        .border_style(method_border_style)
        .text(Method::get_val(uistate.method()));

    f.render_widget(method_input, top_bar_chunks[0]);

    ti_size.x = 12;
    ti_size.width -= 12;

    let mut url_border_style = Style::default();

    if uistate.active_element() == &UIElement::URL {
        url_border_style = url_border_style.fg(Color::Yellow);
    }

    let url_input = TextInput::default()
        .label(String::from(" URL "))
        .borders(Borders::ALL)
        .text(uistate.url())
        .border_style(url_border_style);

    f.render_widget(url_input, top_bar_chunks[1]);

    let send_button = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(send_button, top_bar_chunks[2]);

    let tab_titles = vec![
        Spans::from(vec![Span::raw("Url Params")]),
        Spans::from(vec![Span::raw("Authorization")]),
        Spans::from(vec![Span::raw("Headers")]),
        Spans::from(vec![Span::raw("Body")]),
    ];

    let tab_head = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::NONE))
        .select(0)
        .divider("│")
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(tab_head, tab_chunks[0]);

    let response = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Response ");

    f.render_widget(response, mid_pane[1]);
}

