pub mod widgets;
pub mod state;

use tui::{
    backend::Backend,
    widgets::{ BorderType, Block, Borders, Tabs, Paragraph },
    layout::{ Layout, Constraint, Direction },
    style::{ Color, Style },
    text::{ Span, Spans },
    Frame,
};

use widgets::text_input::TextInput;
use crate::ui::state::{ UiState, Method, UIElement };

pub fn ui_func<B: Backend>(f: &mut Frame<B>, uistate: &mut UiState) {
    let window_size = f.size();

    // Divides window
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(1),
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
            Constraint::Length(8),
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

    // Send button
    let mut send_button_style = Style::default();

    if uistate.active_element() == &UIElement::SendButton {
        send_button_style = send_button_style.fg(Color::Cyan);
    }

    let send_button_outline = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(send_button_style);

    f.render_widget(send_button_outline, top_bar_chunks[2].clone());

    let mut send_rect = top_bar_chunks[2].clone();
    send_rect.x += 2;
    send_rect.width -= 4;
    send_rect.y += 1;

    let send_button_text = Block::default().borders(Borders::NONE)
        .title("Send")
        .style(send_button_style);

    f.render_widget(send_button_text, send_rect);

    // Tab Titles
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

    match uistate.response() {
        Some(content) => {
            let mut rect = mid_pane[1].clone();
            rect.y += 1;
            rect.height -= 2;
            rect.x += 1;
            rect.width -= 2;

            let s: Vec<String> = string_chunks(content, rect.width as usize);

            let response_block = Paragraph::new(string_chunks_to_spans(&s));

            f.render_widget(response_block, rect);
        },
        None => {},
    }

    match uistate.response_status_code() {
        Some(number) => {
            let status_block = Block::default().borders(Borders::NONE)
                .title(vec![
                    Span::styled(
                        " Done ",
                        Style::default().bg(Color::Green).fg(Color::Black)
                    ),
                    Span::raw(format!(" Status: {}", number)),
                ]);

            f.render_widget(status_block, outer_chunks[1]);
        },
        None => {},
    }
}

/**
 * Returns vector of Strings with length upto `max_width`
 */
fn string_chunks(input: &String, max_width: usize) -> Vec<String> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(max_width)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn string_chunks_to_spans<'a>(input: &'a Vec<String>) -> Vec<Spans<'a>> {
    let mut spans: Vec<Spans> = vec![];

    for chunk in input.iter() {
        spans.push(Spans::from(chunk.clone()));
    }

    spans
}

