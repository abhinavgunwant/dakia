//! This module looks after everything in the ui.

pub mod widgets;
pub mod state;
pub mod render;
pub mod calc;

use tui::{
    backend::Backend, style::{ Color, Style }, text::{ Span, Spans }, Frame,
    widgets::{ BorderType, Block, Borders, Tabs, Paragraph },
    layout::{ Layout, Constraint, Direction, Rect },
};

use crate::ui::{
    state::{
        UiState, UIElement, request_tabs::RequestTabs, app_status::AppStatus,
    },
    widgets::text_input::TextInput, render::render_tab_content,
    calc::scrollbar_pos,
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
        .text(String::from(uistate.method().as_str()));

    f.render_widget(method_input, top_bar_chunks[0]);

    ti_size.x = 12;
    ti_size.width -= 12;

    let url_input = TextInput::default()
        .label(String::from(" URL "))
        .borders(Borders::ALL)
        .text(uistate.url())
        .multi_line(false)
        .border_style(Style::default())
        .active_border_style(Style::default().fg(Color::Yellow))
        .width(top_bar_chunks[1].width)
        .cursor_pos(uistate.url_cursor_offset())
        .active(uistate.active_element() == &UIElement::URL);

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

    if uistate.response().initialized() {
        let rect = Rect::new(
            mid_pane[1].x + 1,
            mid_pane[1].y + 1,
            mid_pane[1].width - 2,
            mid_pane[1].height - 2,
        );

        let content_height = rect.height as usize;
        let max_len = rect.width as usize;
        let req_counter = uistate.response().cache_req_counter();

        if uistate.request_counter() != req_counter {
            let mut r_lines: Vec<String> = vec![];

            for line in uistate.response().response().iter() {
                if line.len() > max_len {
                    let mut line_divided = string_chunks(line, max_len);

                    r_lines.append(&mut line_divided);
                } else {
                    r_lines.push(line.clone());
                }
            }

            uistate.response_mut().set_response(r_lines);
            uistate.response_mut().set_cache_req_counter(req_counter);
        }

        let response_text = uistate.response().response();
        let mut response_text_start: usize = 0;
        let mut response_text_end: usize = response_text.len();

        if response_text.len() > content_height {
            response_text_start = uistate.response().scroll_pos() as usize;
            response_text_end = response_text_start + content_height;

            if response_text_end > response_text.len() {
                response_text_end = response_text.len();
            }

            // Draw a scrollbar 
            let scrollbar = Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let scrollbar_rect = Rect::new(rect.width, rect.y, 1, rect.height);

            f.render_widget(scrollbar, scrollbar_rect);

            let scrollbar_thumb = Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::White));

            let (thumb_height, thumb_pos) = scrollbar_pos(
                content_height as u16,
                response_text_end as u16,
                response_text.len() as u16,
            );

            f.render_widget(
                scrollbar_thumb,
                Rect::new(
                    scrollbar_rect.x,
                    scrollbar_rect.y + thumb_pos,
                    1,
                    thumb_height,
            ));
        }

        let response_block = Paragraph::new(string_chunks_to_spans(
            &response_text[response_text_start..response_text_end]
        ));

        f.render_widget(response_block, rect);
    }

    let mut render_status = true;
    let mut status_style = Style::default();
    let mut status_span = Span::raw("");
    // TODO: make the status bar/block show the ui element active/selected
    match uistate.app_status() {
        AppStatus::PROCESSING => {
            status_style = status_style.bg(Color::Yellow).fg(Color::Black);
        },
        AppStatus::DONE => {
            match uistate.response_status_code() {
                Some(req_status_code) => {
                    status_span = Span::raw(
                        format!(" Status: {}", req_status_code)
                    );
                },

                None => {},
            }

            status_style = status_style.bg(Color::Green).fg(Color::Black);
        },
        AppStatus::ERROR => {
            status_style = status_style.bg(Color::Red).fg(Color::White);

            match uistate.app_error() {
                Some(error_string) => {
                    status_span = Span::raw(format!(" {}", error_string));
                },
                None => {},
            }
        },
        AppStatus::STARTUP => { render_status = false; },
    }

    if render_status {
        let status_block = Block::default().borders(Borders::NONE)
            .title(vec![
                Span::styled(
                    format!(" {} ", uistate.app_status().to_str()),
                    status_style
                ),
                status_span,
            ]);

        f.render_widget(status_block, outer_chunks[1]);
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
fn string_chunks_to_spans<'a>(input: &[String]) -> Vec<Spans<'a>> {
    let mut spans: Vec<Spans> = vec![];

    for chunk in input.iter() {
        spans.push(Spans::from(chunk.clone()));
    }

    spans
}

