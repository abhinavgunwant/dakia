pub mod kv_tab;

use std::io::Error;
use log::info;

use crossterm::event::{ self, Event, KeyCode, KeyModifiers };
use crate::{
    ui::state::{
        UiState, InputMode, EditorMode, UIElement,
        request_tabs::RequestTabs, kv_data::KVData, app_status::AppStatus,
        text_edit::TextEditMoveDirection, body::{ BodyUIElement, BodyContent },
    },
    api::call_api,
    user_input::kv_tab::{ KVTabOperation, process_kv_tab_input },
};

/// User interaction related code...
/// Returning `Ok(true)` shall exit the program.
pub fn process_user_input(uistate: &mut UiState) -> Result<bool, Error> {
    if uistate.clone().editor_mode() == EditorMode::Normal
        || uistate.clone().input_mode() == InputMode::Normal
    {
        if let Event::Key(key) = event::read()? {
            let mut update_url: bool = false;

            if key.code == KeyCode::Esc {
                info!("Exiting normally");
                return Ok(true);
            }

            match uistate.active_element() {
                UIElement::URL => {
                    match key.code {
                        KeyCode::Char(c) => {
                            uistate.append_url(c);
                            let url = uistate.url();

                            uistate.url_deconst_mut().update(url);
                        }

                        KeyCode::Backspace => {
                            uistate.pop_url();
                        }

                        KeyCode::Enter => {
                            uistate.set_app_status(AppStatus::PROCESSING);

                            match call_api(uistate) {
                                Ok(()) => {},
                                Err(_e) => {},
                            }
                        }

                        KeyCode::Right => { uistate.url_cursor_right(); }
                        KeyCode::Left => { uistate.url_cursor_left(); }

                        _ => {}
                    }
                }

                UIElement::RequestTabsElem => {
                    let row = uistate.query_params_ui().active_row();
                    let col = uistate.query_params_ui().active_col();

                    match uistate.active_request_tab() {
                        RequestTabs::UrlParams => {
                            process_kv_tab_input(key, row, col, |op| {
                                match op {
                                    KVTabOperation::Insert(pos) => {
                                        uistate.insert_url_param(
                                            pos,
                                            KVData::default()
                                        );
                                    }

                                    KVTabOperation::Remove(pos) => {
                                        uistate.remove_url_param(pos);

                                        if uistate.url_deconst()
                                            .query_params().len() == 0
                                        {
                                            uistate.insert_url_param(
                                                0,
                                                KVData::default()
                                            );
                                        }
                                    }

                                    KVTabOperation::MoveColumn(col) => {
                                        uistate.query_params_ui_mut()
                                            .set_active_col(col);
                                    }

                                    KVTabOperation::MoveRow(row) => {
                                        uistate.query_params_ui_mut()
                                            .set_active_row(row);
                                    }

                                    KVTabOperation::AppendText(c) => {
                                        match uistate.url_deconst_mut().get_param(row) {
                                            Some (active_qparam) => {
                                                if col == 0 {
                                                    let mut key = active_qparam.key();
                                                    key.push(c);

                                                    active_qparam.set_key(key);
                                                }

                                                if col == 1 {
                                                    let mut val = active_qparam.value();
                                                    val.push(c);

                                                    active_qparam.set_value(val);
                                                }
                                            }

                                            None => {}
                                        }
                                    }

                                    KVTabOperation::PopText() => {
                                        match uistate.url_deconst_mut().get_param(row) {
                                            Some (qparam) => {
                                                if col == 0 {
                                                    let mut key = qparam.key();
                                                    key.pop();

                                                    qparam.set_key(key);
                                                }

                                                if col == 1 {
                                                    let mut val = qparam.value();
                                                    val.pop();

                                                    qparam.set_value(val);
                                                }
                                            }

                                            None => {}
                                        }
                                    }
                                }},
                                | update | { update_url = update; },
                            );
                        }

                        RequestTabs::Headers => {
                            let row = uistate.request_headers_ui().active_row();
                            let col = uistate.request_headers_ui().active_col();

                            process_kv_tab_input(key, row, col, |op| {
                                match op {
                                    KVTabOperation::Insert(pos) => {
                                        uistate.insert_header(
                                            pos,
                                            KVData::default()
                                        );
                                    }

                                    KVTabOperation::Remove(pos) => {
                                        uistate.remove_header(pos);
                                    }

                                    KVTabOperation::MoveColumn(col) => {
                                        uistate.request_headers_ui_mut()
                                            .set_active_col(col);
                                    }

                                    KVTabOperation::MoveRow(row) => {
                                        uistate.request_headers_ui_mut()
                                            .set_active_row(row);
                                    }

                                    KVTabOperation::AppendText(c) => {
                                        match uistate.request_headers_mut().get_mut(row as usize) {
                                            Some (active_header) => {
                                                if col == 0 {
                                                    let mut key = active_header.key();
                                                    key.push(c);

                                                    active_header.set_key(key);
                                                }

                                                if col == 1 {
                                                    let mut val = active_header.value();
                                                    val.push(c);

                                                    active_header.set_value(val);
                                                }
                                            }

                                            None => {}
                                        }
                                    }

                                    KVTabOperation::PopText() => {
                                        match uistate.request_headers_mut().get_mut(row as usize) {
                                            Some (active_header) => {
                                                if col == 0 {
                                                    let mut key = active_header.key();
                                                    key.pop();

                                                    active_header.set_key(key);
                                                }

                                                if col == 1 {
                                                    let mut val = active_header.value();
                                                    val.pop();

                                                    active_header.set_value(val);
                                                }
                                            }

                                            None => {}
                                        }
                                    }
                                }},
                                |_update| {}
                            );
                        }

                        RequestTabs::Body => {
                            let body = uistate.body_mut();

                            match body.clone().active_body_element() {
                                BodyUIElement::TextArea => {
                                    match key.code {
                                        KeyCode::Up => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::ContentType(false)
                                                    );
                                            } else {
                                                uistate.body_mut()
                                                    .text_data_mut()
                                                    .move_cursor(
                                                        TextEditMoveDirection::Up,
                                                        false,
                                                        key.modifiers == KeyModifiers::SHIFT,
                                                    );
                                            }
                                        }
                                        
                                        KeyCode::Down => {
                                            uistate.body_mut()
                                                .text_data_mut()
                                                .move_cursor(
                                                    TextEditMoveDirection::Down,
                                                    false,
                                                    key.modifiers == KeyModifiers::SHIFT,
                                                );
                                        }

                                        KeyCode::Left => {
                                            uistate.body_mut()
                                                .text_data_mut()
                                                .move_cursor(
                                                    TextEditMoveDirection::Left,
                                                    key.modifiers.contains(KeyModifiers::CONTROL),
                                                    key.modifiers.contains(KeyModifiers::SHIFT),
                                                );
                                        }

                                        KeyCode::Right => {
                                            uistate.body_mut()
                                                .text_data_mut()
                                                .move_cursor(
                                                    TextEditMoveDirection::Right,
                                                    key.modifiers.contains(KeyModifiers::CONTROL),
                                                    key.modifiers.contains(KeyModifiers::SHIFT),
                                                );
                                        }

                                        KeyCode::End => {
                                            uistate.body_mut().text_data_mut()
                                                .move_cursor(
                                                    TextEditMoveDirection::End,
                                                    false,
                                                    key.modifiers == KeyModifiers::SHIFT,
                                                );
                                        }

                                        KeyCode::Home => {
                                            uistate.body_mut().text_data_mut()
                                                .move_cursor(
                                                    TextEditMoveDirection::Home,
                                                    false,
                                                    key.modifiers == KeyModifiers::SHIFT,
                                                );
                                        }

                                        KeyCode::Enter => {
                                            uistate.body_mut().text_data_mut().new_line();
                                        }

                                        KeyCode::Char(c) => {
                                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                                match c {
                                                    'a' | 'A' => {
                                                        uistate.body_mut()
                                                            .text_data_mut()
                                                            .select_all();
                                                    }

                                                    'c' | 'C' => {
                                                        uistate.body_mut()
                                                            .text_data_mut()
                                                            .copy_selected();
                                                    }

                                                    'v' | 'V' => {
                                                        uistate.body_mut()
                                                            .text_data_mut()
                                                            .paste();
                                                    }

                                                    _ => {}
                                                }
                                            } else {
                                                uistate.body_mut().text_data_mut()
                                                    .insert_char(c);
                                            }
                                        }

                                        KeyCode::Backspace => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .text_data_mut()
                                                    .delete_word();
                                            } else if uistate.body().text_data().selecting() {
                                                uistate.body_mut()
                                                    .text_data_mut()
                                                    .delete_selected();
                                            } else {
                                                uistate.body_mut()
                                                    .text_data_mut()
                                                    .delete_char();
                                            }
                                        }

                                        KeyCode::Delete => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut().text_data_mut()
                                                    .delete_word_to_right();
                                            } else if uistate.body().text_data().selecting() {
                                                uistate.body_mut()
                                                    .text_data_mut()
                                                    .delete_selected();
                                            } else {
                                                uistate.body_mut().text_data_mut()
                                                    .delete_char_to_right();
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                BodyUIElement::ContentType(opened) => {
                                    match key.code {
//                                        KeyCode::Right => {
//                                            if key.modifiers == KeyModifiers::CONTROL {
//                                                match uistate.body().body_content() {
//                                                    BodyContent::Raw(_) => {
//                                                        uistate.body_mut()
//                                                            .set_active_body_element(
//                                                                BodyUIElement::RawContentType(false)
//                                                            );
//                                                    }
//
//                                                    BodyContent::FormData
//                                                        | BodyContent::FormURLEncoded => {
//                                                        uistate.body_mut()
//                                                            .set_active_body_element(
//                                                                BodyUIElement::TextArea
//                                                            );
//                                                    }
//
//                                                    _ => {}
//                                                }
//                                            }
//                                        }
//
//                                        KeyCode::Left => {
//                                            if key.modifiers == KeyModifiers::CONTROL {
//                                                if *uistate.body().body_content() != BodyContent::NONE {
//                                                    uistate.body_mut()
//                                                        .set_active_body_element(
//                                                            BodyUIElement::TextArea
//                                                        );
//                                                }
//                                            }
//                                        }

                                        KeyCode::Enter => {
                                            if *opened {
                                                let body = uistate.body_mut();
                                                let index = *body.body_content_sel_index() as usize;
                                                let selected_option: String = body.body_content_options()[index].clone();

                                                body.set_body_content(
                                                    BodyContent::from_string(selected_option)
                                                );

                                                body.set_active_body_element(
                                                    BodyUIElement::ContentType(false)
                                                );
                                            } else {
                                                uistate.body_mut().set_active_body_element(
                                                    BodyUIElement::ContentType(true)
                                                );
                                            }
                                        }

                                        KeyCode::Up => {
                                            if *opened {
                                                let s = *uistate.body().body_content_sel_index();
                                                let offset = *uistate.body().body_content_scroll_offset();

                                                if s > 0 {
                                                    if offset == s {
                                                        uistate.body_mut().set_body_content_scroll_offset(offset - 1);
                                                    }

                                                    uistate.body_mut().set_body_content_sel_index(s - 1);
                                                }
                                            }
                                        }

                                        KeyCode::Down => {
                                            if *opened {
                                                let current_selection = *uistate.body().body_content_sel_index();
                                                let s = current_selection + 1;
                                                let offset = *uistate.body().body_content_scroll_offset();
                                                let selection_at_bottom: bool = (current_selection - offset) == 4;

                                                if s < uistate.body().body_content_options().len() as u8 {
                                                    if selection_at_bottom {
                                                        uistate.body_mut().set_body_content_scroll_offset(offset + 1);
                                                    }

                                                    uistate.body_mut().set_body_content_sel_index(s);
                                                }
                                            } else if key.modifiers == KeyModifiers::CONTROL {
                                                match body.body_content() {
                                                    BodyContent::FormData | BodyContent::FormURLEncoded => {
                                                        body.set_active_body_element(BodyUIElement::KVArea);
                                                    }

                                                    BodyContent::Text | BodyContent::Html | BodyContent::Xml => {
                                                        body.set_active_body_element(BodyUIElement::TextArea);
                                                    }

                                                    _ => {}
                                                }
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                BodyUIElement::KVArea => {
                                    match body.body_content() {
                                        BodyContent::FormData | BodyContent::FormURLEncoded => {
                                            let row = body.kv_tab_state().active_row();
                                            let col = body.kv_tab_state().active_col();

                                            if row == 0 && key.code == KeyCode::Up {
                                                body.set_active_body_element(BodyUIElement::ContentType(false));
                                                return Ok(false);
                                            }

                                            process_kv_tab_input(key, row, col, |op| {
                                                match op {
                                                    KVTabOperation::Insert(pos) => {
                                                        body.kv_data_mut().insert(pos as usize, KVData::default());
                                                    }

                                                    KVTabOperation::Remove(pos) => {
                                                        let kv_vec = body.kv_data_mut();

                                                        kv_vec.remove(pos as usize);

                                                        if kv_vec.len() == 0 {
                                                            kv_vec.insert(0, KVData::default());
                                                        }
                                                    }

                                                    KVTabOperation::MoveColumn(col) => {
                                                        body.kv_tab_state_mut().set_active_col(col);
                                                    }

                                                    KVTabOperation::MoveRow(row) => {
                                                        body.kv_tab_state_mut().set_active_row(row);
                                                    }

                                                    KVTabOperation::AppendText(c) => {

                                                        let vec_mut = body.kv_data_mut();

                                                        match vec_mut.get_mut(row as usize) {
                                                            Some(kv_data) => {
                                                                if col == 0 {
                                                                    let mut key = kv_data.key();
                                                                    key.push(c);

                                                                    kv_data.set_key(key);
                                                                }

                                                                if col == 1 {
                                                                    let mut val = kv_data.value();
                                                                    val.push(c);

                                                                    kv_data.set_value(val);
                                                                }
                                                            }

                                                            None => {}
                                                        }
                                                    }

                                                    KVTabOperation::PopText() => {
                                                        let row = body.kv_tab_state().active_row();
                                                        let col = body.kv_tab_state().active_col();

                                                        let vec_mut = body.kv_data_mut();

                                                        match vec_mut.get_mut(row as usize) {
                                                            Some(kv_data) => {
                                                                if col == 0 {
                                                                    let mut key = kv_data.key();
                                                                    key.pop();

                                                                    kv_data.set_key(key);
                                                                }

                                                                if col == 1 {
                                                                    let mut val = kv_data.value();
                                                                    val.pop();

                                                                    kv_data.set_value(val);
                                                                }
                                                            }

                                                            None => {}
                                                        }
                                                    }
                                                }},
                                                |_update| {}
                                            );
                                        }
                                        _ => {}
                                    }
                                }

//                                BodyUIElement::RawContentType(opened) => {
//                                    match key.code {
//                                        KeyCode::Right => {
//                                            if key.modifiers == KeyModifiers::CONTROL {
//                                                uistate.body_mut()
//                                                    .set_active_body_element(
//                                                        BodyUIElement::TextArea
//                                                    );
//                                            }
//                                        }
//
//                                        KeyCode::Left => {
//                                            if key.modifiers == KeyModifiers::CONTROL {
//                                                uistate.body_mut()
//                                                    .set_active_body_element(
//                                                        BodyUIElement::ContentType(false)
//                                                    );
//                                            }
//                                        }
//
//                                        KeyCode::Up => {
//                                            if *opened {
//                                                let s = *uistate.body().raw_body_content_sel_index();
//
//                                                if s > 0 {
//                                                    uistate.body_mut().set_raw_body_content_sel_index(s - 1);
//                                                }
//                                            }
//                                        }
//
//                                        KeyCode::Down => {
//                                            if *opened {
//                                                let s = *uistate.body().raw_body_content_sel_index() + 1;
//
//                                                if s < uistate.body().raw_body_content_options().len() as u8 {
//                                                    uistate.body_mut().set_raw_body_content_sel_index(s);
//                                                }
//                                            } else if key.modifiers == KeyModifiers::CONTROL {
//                                                if *uistate.body().body_content() != BodyContent::NONE {
//                                                    uistate.body_mut()
//                                                        .set_active_body_element(
//                                                            BodyUIElement::TextArea
//                                                        );
//                                                }
//                                            }
//                                        }
//
//                                        KeyCode::Enter => {
//                                            if *opened {
//                                                let body = uistate.body_mut();
//                                                let index = *body.raw_body_content_sel_index() as usize;
//                                                let selected_option: String = body.raw_body_content_options()[index].clone();
//
//                                                body.set_raw_body_content(
//                                                    RawBodyContentType::from_string(selected_option)
//                                                );
//
//                                                uistate.body_mut()
//                                                    .set_active_body_element(
//                                                        BodyUIElement
//                                                            ::RawContentType(false)
//                                                    );
//                                            } else {
//                                                uistate.body_mut().set_active_body_element(
//                                                    BodyUIElement
//                                                        ::RawContentType(true)
//                                                );
//                                            }
//                                        }
//
//                                        _ => {}
//                                    }
//                                }
                            }
                        }

                        _ => {}
                    }
                }

                UIElement::RequestTabsHead => {
                    match key.code {
                        KeyCode::Right => {
                            uistate.activate_next_req_tab();
                        }

                        KeyCode::Left => {
                            uistate.activate_previous_req_tab();
                        }

                        _ => {}
                    }
                }

                UIElement::ResponseArea => {
                    let pos = uistate.response().scroll_pos();
                    let scroll_by: u16;

                    if key.modifiers == KeyModifiers::CONTROL {
                        scroll_by = 4;
                    } else {
                        scroll_by = 1;
                    }

                    match key.code {
                        KeyCode::Up => {
                            let new_pos: i32 = (pos as i32) - scroll_by as i32;

                            if new_pos >= 0 {
                                uistate.response_mut()
                                    .set_scroll_pos(new_pos as u16);
                            } else {
                                uistate.response_mut().set_scroll_pos(0);
                            }
                        }

                        KeyCode::Down => {
                            let new_pos = pos + scroll_by;

                            if new_pos < uistate.response().response().len() as u16 {
                                uistate.response_mut().set_scroll_pos(new_pos);
                            }
                        }

                        _ => {}
                    }
                }

                UIElement::SendButton => {
                    match key.code {
                        KeyCode::Enter => {
                            let req_counter = uistate.request_counter();
                            info!(
                                "Making call (#{}) to: {}",
                                req_counter,
                                uistate.url_deconst().to_string()
                            );

                            match call_api(uistate) {
                                Ok(_) => {
                                    info!("No Errors for #{}", req_counter);
                                }

                                Err(msg) => {
                                    let source: String;

                                    match msg.source() {
                                        Some(err) => source = err.to_string(),
                                        None => source = String::default(),
                                    }

                                    info!("{}", msg.to_string());

                                    uistate.set_app_error(msg.to_string());
                                    uistate.set_app_status(AppStatus::ERROR);
                                }
                            }
                        }

                        _ => {}
                    }
                }

                UIElement::Method => {
                    match key.code {
                        KeyCode::Char(c) => {
                            match c.to_digit(10) {
                                Some(num) => {
                                    if num > 0 {
                                        uistate.set_method_from_val(
                                            (num-1) as u8
                                        );
                                    }
                                }

                                None => { uistate.set_method_from_char(c); }
                            }
                        }

                        _ => {}
                    }
                }
            }

            // "global" keys
            match key.code {
                KeyCode::Tab => {
                    uistate.activate_next_element();
                },
                KeyCode::BackTab => {
                    uistate.activate_previous_element();
                },
                _ => {  },
            };

            if update_url {
                uistate.set_url(
                    uistate.url_deconst().to_string()
                );
            }
        }
    }

    Ok(false)
}

