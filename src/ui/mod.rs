//! This module looks after everything in the ui.

pub mod widgets;
pub mod state;

use tui::{
    backend::Backend,
    widgets::{ BorderType, Block, Borders, Tabs, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
    style::{ Color, Style },
    text::{ Span, Spans },
    Frame,
};

use widgets::{
    text_input::TextInput,
    label::Label,
};
use crate::ui::state::{
    UiState, Method, UIElement, request_tabs::RequestTabs,
};

/// Main rendering function called whenever the ui has to be re-rendered.
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
        .constraints([Constraint::Length(1), Constraint::Min(10)].as_ref())
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
        .text(Method::get_str_label(uistate.method()));

    f.render_widget(method_input, top_bar_chunks[0]);

    ti_size.x = 12;
    ti_size.width -= 12;

    //let url_border_style = Style::default();

    let url_input = TextInput::default()
        .label(String::from(" URL "))
        .borders(Borders::ALL)
        .text(uistate.url())
        .border_style(Style::default())
        .active_border_style(Style::default().fg(Color::Yellow))
        .active(uistate.active_element() == &UIElement::URL);

//    if uistate.active_element() == &UIElement::URL {
//        url_input = url_input.border_style(url_border_style.fg(Color::Yellow));
//    }

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
    send_rect.height = 1;

    let send_button_text = Block::default().borders(Borders::NONE)
        .title("Send")
        .style(send_button_style);

    f.render_widget(send_button_text, send_rect);

    // Tab Titles
    let mut tab_titles = vec![];

    for req_tabs in RequestTabs::iter() {
        tab_titles.push(
            Spans::from(vec![Span::raw(req_tabs.get_str_label())])
        );
    }

    let mut tab_head_style = Style::default().fg(Color::Gray);
    let mut tab_head_active_style = Style::default().fg(Color::Yellow);

    if uistate.active_element() == &UIElement::RequestTabsHead {
        tab_head_style = tab_head_style.fg(Color::Black).bg(Color::Yellow);
        tab_head_active_style = tab_head_active_style.fg(Color::Black)
            .bg(Color::Cyan);
    }

    let tab_head = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::NONE))
        .select(uistate.active_request_tab().get_val() as usize)
        .divider("│")
        .style(tab_head_style)
        .highlight_style(tab_head_active_style);

    f.render_widget(tab_head, tab_chunks[0]);

    render_tab_content(f, uistate, tab_chunks[1]);

    let mut response_style = Style::default().fg(Color::Gray);

    if uistate.active_element() == &UIElement::ResponseArea {
        response_style = response_style.fg(Color::Yellow);
    }

    let response = Block::default().borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(response_style)
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

    // TODO: make the status bar/block show the ui element active/selected
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

/// Renders tab content
fn render_tab_content<B: Backend>(f: &mut Frame<B>, uistate: &mut UiState, rect: Rect) {
    let mut tab_style = Style::default();

    if uistate.active_element() == &UIElement::RequestTabsElem {
        tab_style = tab_style.bg(Color::Reset).fg(Color::Yellow);
    }

    let tab_content_box = Block::default()
        .borders(Borders::ALL)
        .style(tab_style)
        .border_type(BorderType::Rounded);

    f.render_widget(tab_content_box, rect);

    let mut rect_inset = rect;
    rect_inset.x += 2;
    rect_inset.y += 1;
    rect_inset.width -= 4;
    rect_inset.height -= 2;

    match uistate.active_request_tab() {
        RequestTabs::UrlParams => {
            let params = uistate.query_params_ui();
            let mut content_rect = rect_inset.clone();
            content_rect.height = 3;

            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(10),
                ].as_ref())
                .split(content_rect);

            let param_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(content_chunks[0]);

            let param_actions_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(content_chunks[1]);
            
            //for (i, param) in params.params().iter().enumerate() {
            for (i, param) in uistate.url_deconst().query_params().iter().enumerate() {
                let mut name_rect = param_chunks[0];
                let mut value_rect = param_chunks[1];
                let mut add_param_chunk = param_actions_chunk[0];
                let mut remove_param_chunk = param_actions_chunk[1];
                let row_active: bool;

                if params.active_param_row() == i as u16 {
                    row_active = true;
                } else {
                    row_active = false;
                }

                if i > 0 {
                    name_rect.y += (3 * i) as u16;
                    value_rect.y += (3 * i) as u16;
                    add_param_chunk.y += (3 * i) as u16;
                    remove_param_chunk.y += (3 * i) as u16;
                }

                let mut param_name_style = Style::default().fg(Color::Gray);

                if row_active && params.active_param_col() == 0 {
                    param_name_style = param_name_style.fg(Color::Yellow);
                }

                let param_name = TextInput::default()
                    .label(String::from(" Name "))
                    .borders(Borders::ALL)
                    .text(param.name())
                    .border_style(param_name_style);

                f.render_widget(param_name, name_rect);

                let mut param_value_style = Style::default().fg(Color::Gray);

                if row_active && params.active_param_col() == 1 {
                    param_value_style = param_value_style.fg(Color::Yellow);
                }

                let param_value = TextInput::default()
                    .label(String::from(" Value "))
                    .borders(Borders::ALL)
                    .text(param.value())
                    .border_style(param_value_style);

                f.render_widget(param_value, value_rect);

                let mut param_add_style = Style::default().fg(Color::White);

                if row_active && params.active_param_col() == 2 {
                    param_add_style = param_add_style.fg(Color::Cyan);
                }

                let action_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);

                let add_param = action_block.clone().style(param_add_style);

                f.render_widget(add_param, add_param_chunk);
                add_param_chunk.x += 2;
                add_param_chunk.width -= 2;
                add_param_chunk.y += 1;
                f.render_widget(
                    Label::default().text("+").style(param_add_style),
                    add_param_chunk
                );

                let mut param_remove_style = Style::default().fg(Color::White);

                if row_active && params.active_param_col() == 3 {
                    param_remove_style = param_remove_style.fg(Color::Cyan);
                }

                let remove_param = action_block.style(param_remove_style);

                f.render_widget(remove_param, remove_param_chunk);
                remove_param_chunk.x += 2;
                remove_param_chunk.y += 1;
                f.render_widget(
                    Label::default().text("-").style(param_remove_style),
                    remove_param_chunk,
                );
            }
        },
        RequestTabs::Authorization => {
            let content = Paragraph::new(String::from("Authorization"));
            f.render_widget(content, rect_inset);
        },
        RequestTabs::Headers => {
            let content = Paragraph::new(String::from("Headers"));
            f.render_widget(content, rect_inset);
        },
        RequestTabs::Body => {
            let content = Paragraph::new(String::from("Body"));
            f.render_widget(content, rect_inset);
        },
    }
}

/// Returns vector of Strings with length upto `max_width`.
/// Used to "wrap" the text.
fn string_chunks(input: &String, max_width: usize) -> Vec<String> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(max_width)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

/// Converts the string chunks to a vector of `Spans`.
fn string_chunks_to_spans<'a>(input: &'a Vec<String>) -> Vec<Spans<'a>> {
    let mut spans: Vec<Spans> = vec![];

    for chunk in input.iter() {
        spans.push(Spans::from(chunk.clone()));
    }

    spans
}

