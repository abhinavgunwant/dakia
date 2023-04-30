use std::io::Error;

use crossterm::event::{ self, Event, KeyCode, KeyModifiers };
use crate::{
    ui::state::{
        UiState, InputMode, EditorMode, UIElement, Method,
        request_tabs::RequestTabs,
        query_param::QueryParam,
    },
    api::call_api,
};


/// User interaction related code...
/// Returning `Ok(true)` shall exit the program.
pub fn process_user_input(uistate: &mut UiState) -> Result<bool, Error> {
    if uistate.clone().editor_mode() == EditorMode::Normal
        || uistate.clone().input_mode() == InputMode::Normal
    {
        if let Event::Key(key) = event::read()? {
            let mut update_url: bool = false;

            match key.code {
                KeyCode::Esc => { return Ok(true); },

                KeyCode::Char(c) => {
                    match uistate.active_element() {
                        UIElement::URL => {
                            uistate.append_url(c);
                            let url = uistate.url();

                            uistate.url_deconst_mut()
                                .update(url);
                        },

                        UIElement::Method => {
                            match c.to_digit(10) {
                                Some(mut num) => {
                                    if num > 0 {
                                        num -= 1;
                                        uistate.set_method(
                                            Method::from_val(num as u8)
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
                        UIElement::SendButton => {},
                        UIElement::RequestTabsElem => {
                            let uparams = uistate.query_params_ui();
                            let active_col = uparams.active_param_col();
                            let active_row = uparams.active_param_row();

                            match uistate.url_deconst_mut().get_param(active_row) {
                                Some (active_qparam) => {
                                    if active_col == 0 {
                                        let mut name_str = active_qparam.name();
                                        name_str.push(c);

                                        active_qparam.set_name(name_str);
                                        update_url = true;
                                    }

                                    if active_col == 1 {
                                        let mut value_str = active_qparam.value();
                                        value_str.push(c);

                                        active_qparam.set_value(value_str);
                                        update_url = true;
                                    }
                                },
                                None => {},
                            }
                        },
                        _ => {},
                    }
                },
                KeyCode::Backspace => {
                    match uistate.active_element() {
                        UIElement::URL => {
                            uistate.pop_url();
                        },

                        UIElement::RequestTabsElem => {
                            let uparams = uistate.query_params_ui();
                            let col = uparams.active_param_col();
                            let row = uparams.active_param_row();

                            if col == 0 {
                                match uistate.url_deconst_mut().get_param(row) {
                                    Some (qparam) => {
                                        let mut name_str = qparam.name();
                                        name_str.pop();

                                        qparam.set_name(name_str);
                                        update_url = true;
                                    },
                                    None => {},
                                }
                            }

                            if col == 1 {
                                match uistate.url_deconst_mut().get_param(row) {
                                    Some (qparam) => {
                                        let mut value_str = qparam.value();
                                        value_str.pop();

                                        qparam.set_value(value_str);
                                        update_url = true;
                                    },
                                    None => {},
                                }
                            }
                        },

                        _ => {},
                    }
                },
                KeyCode::Enter => {
                    match uistate.active_element() {
                        UIElement::URL => {
                            match call_api(uistate) {
                                Ok(()) => {},
                                Err(_e) => {},
                            }
                        },
                        UIElement::SendButton => {
                            call_api(uistate).unwrap();
                        },
                        UIElement::RequestTabsElem => {
                            match uistate.active_request_tab() {
                                RequestTabs::UrlParams => {
                                    let mut p = uistate.query_params_ui();
                                    let url_obj = uistate.url_deconst_mut();
                                    let row = p.active_param_row();
                                    let col = p.active_param_col();

                                    // The "+" or add parameter button
                                    if col == 2 {
                                        let new_qparam = QueryParam::default();
                                        url_obj.insert_param(row+1, new_qparam);
                                        update_url = true;
                                    }

                                    // The "-" or remove parameter button
                                    if col == 3 {
                                        url_obj.remove_param(row);

                                        if row > 0 {
                                            p.set_active_param_row(row - 1);
                                        }
                                        update_url = true;

                                        if url_obj.query_params().len() == 0 {
                                            url_obj.insert_param(
                                                0,
                                                QueryParam::default()
                                            );
                                        }
                                    }
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                }
                KeyCode::Tab => {
                    uistate.activate_next_element();
                },
                KeyCode::BackTab => {
                    uistate.activate_previous_element();
                },
                KeyCode::Up => {
                    if *uistate.active_element() == UIElement::RequestTabsElem {
                        match uistate.active_request_tab() {
                            RequestTabs::UrlParams => {
                                let mut apr = uistate.query_params_ui_mut()
                                    .active_param_row();

                                if apr != 0 {
                                    apr -= 1;
                                    uistate.query_params_ui_mut().set_active_param_row(apr);
                                }
                            },
                            _ => {},
                        }
                    }
                },
                KeyCode::Right => {
                    match uistate.active_element() {
                        UIElement::RequestTabsElem => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                match uistate.active_request_tab() {
                                    RequestTabs::UrlParams => {
                                        let params = uistate.query_params_ui_mut();
                                        let apr = params.active_param_col();

                                        if apr < 3 {
                                            params.set_active_param_col(apr + 1);
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        },
                        UIElement::RequestTabsHead => {
                            uistate.activate_next_req_tab();
                        },
                        _ => {},
                    }
                },
                KeyCode::Left => {
                    match uistate.active_element() {
                        UIElement::RequestTabsElem => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                match uistate.active_request_tab() {
                                    RequestTabs::UrlParams => {
                                        let params = uistate.query_params_ui_mut();
                                        let apr = params.active_param_col();

                                        if apr > 0 {
                                            params.set_active_param_col(apr - 1);
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        },
                        UIElement::RequestTabsHead => {
                            uistate.activate_previous_req_tab();
                        },
                        _ => {},
                    }
                },
                KeyCode::Down => {
                    if *uistate.active_element() == UIElement::RequestTabsElem {
                        match uistate.active_request_tab() {
                            RequestTabs::UrlParams => {
                                if key.modifiers == KeyModifiers::CONTROL {
                                    let mut apr = uistate.query_params_ui_mut()
                                        .active_param_row();

                                    if apr <= 1000 {
                                        apr += 1;

                                        if apr < uistate.url_deconst().query_params().len() as u16 {
                                            uistate.query_params_ui_mut().set_active_param_row(apr);
                                        }
                                    }
                                }
                            },
                            _ => {},
                        }
                    }
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

