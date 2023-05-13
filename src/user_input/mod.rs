pub mod kv_tab;

use std::io::Error;

use crossterm::event::{ self, Event, KeyCode, KeyModifiers };
use reqwest::Method;
use crate::{
    ui::state::{
        UiState, InputMode, EditorMode, UIElement,
        request_tabs::RequestTabs, kv_data::KVData, app_status::AppStatus, body::BodyUIElement,
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
                            match uistate.body().active_body_element() {
                                BodyUIElement::TextArea => {
                                    match key.code {
                                        KeyCode::Up => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::ContentType(false)
                                                    );
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                BodyUIElement::ContentType(opened) => {
                                    match key.code {
                                        KeyCode::Right => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::RawContentType(false)
                                                    );
                                            }
                                        }

                                        KeyCode::Left => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::TextArea
                                                    );
                                            }
                                        }

                                        KeyCode::Enter => {
                                            if *opened {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement
                                                            ::ContentType(false)
                                                    );
                                            } else {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement
                                                            ::ContentType(true)
                                                    );
                                            }
                                        }

                                        KeyCode::Up => {
                                            if *opened {
                                                let s = *uistate.body().body_content_sel_index();

                                                if s > 0 {
                                                    uistate.body_mut().set_body_content_sel_index(s - 1);
                                                }
                                            }
                                        }

                                        KeyCode::Down => {
                                            if *opened {
                                                let s = *uistate.body().body_content_sel_index() + 1;

                                                if s < uistate.body().body_content_options().len() as u8 {
                                                    uistate.body_mut().set_body_content_sel_index(s);
                                                }
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                BodyUIElement::RawContentType(active) => {
                                    match key.code {
                                        KeyCode::Right => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::TextArea
                                                    );
                                            }
                                        }

                                        KeyCode::Left => {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                uistate.body_mut()
                                                    .set_active_body_element(
                                                        BodyUIElement::ContentType(false)
                                                    );
                                            }
                                        }

                                        _ => {}
                                    }
                                }
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
                            call_api(uistate).unwrap();
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

