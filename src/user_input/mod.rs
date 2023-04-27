use std::io::Error;

use crossterm::event::{ self, Event, KeyCode };
use crate::{
    ui::state::{
        UiState, InputMode, EditorMode, UIElement, Method,
        request_tabs::RequestTabs,
        url_params::Param,
    },
    api::call_api,
    utils::url::url_with_params
};


/// User interaction related code...
/// Returning `Ok(true)` shall exit the program.
pub fn process_user_input(uistate: &mut UiState) -> Result<bool, Error> {
    if uistate.clone().editor_mode() == EditorMode::Normal
        || uistate.clone().input_mode() == InputMode::Normal
    {
        if let Event::Key(key) = event::read()? {
            let mut url_params_changed: bool = false;

            match key.code {
                KeyCode::Esc => { return Ok(true); },

                KeyCode::Char(c) => {
                    match uistate.active_element() {
                        UIElement::URL => { uistate.append_url(c); },

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
                            let uparams = uistate.url_params_mut();

                            if uparams.active_param_col() == 0 {
                                let mut s = uparams.get_param(
                                    uparams.active_param_row()
                                ).clone().name();

                                s.push(c);

                                uparams.param_name_update(
                                    uparams.active_param_row(),
                                    s,
                                );

                                // TODO: uncomment when stable
                                // url_params_changed = true;
                            }

                            if uparams.active_param_col() == 1 {
                                let mut s = uparams.get_param(
                                    uparams.active_param_row()
                                ).clone().value();

                                s.push(c);

                                uparams.param_value_update(
                                    uparams.active_param_row(),
                                    s,
                                );

                                // TODO: uncomment when stable
                                // url_params_changed = true;
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
                            let uparams = uistate.url_params_mut();

                            if uparams.active_param_col() == 0 {
                                let mut s = uparams.get_param(
                                    uparams.active_param_row()
                                ).clone().name();

                                s.pop();

                                uparams.param_name_update(
                                    uparams.active_param_row(),
                                    s,
                                );

                                // TODO: uncomment when stable
                                // url_params_changed = true;
                            }

                            if uparams.active_param_col() == 1 {
                                let mut s = uparams.get_param(
                                    uparams.active_param_row()
                                ).clone().value();

                                s.pop();

                                uparams.param_value_update(
                                    uparams.active_param_row(),
                                    s,
                                );

                                // TODO: uncomment when stable
                                // url_params_changed = true;
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
                                    let p = uistate.url_params_mut();
                                    let row = p.active_param_row();
                                    let col = p.active_param_col();

                                    // The "+" or add parameter button
                                    if col == 2 {
                                        let new_param = Param::default();
                                        p.insert_param(row+1, new_param);
                                    }

                                    // The "-" or remove parameter button
                                    if col == 3 {
                                        p.remove_param(row);

                                        if row > 0 {
                                            p.set_active_param_row(row - 1);
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
                                let mut apr = uistate.url_params_mut()
                                    .active_param_row();

                                if apr != 0 {
                                    apr -= 1;
                                    uistate.url_params_mut().set_active_param_row(apr);
                                }
                            },
                            _ => {},
                        }
                    }
                },
                KeyCode::Right => {
                    match uistate.active_element() {
                        UIElement::RequestTabsElem => {
                            match uistate.active_request_tab() {
                                RequestTabs::UrlParams => {
                                    let params = uistate.url_params_mut();
                                    let apr = params.active_param_col();

                                    if apr < 4 {
                                        params.set_active_param_col(apr + 1);
                                    }
                                },
                                _ => {},
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
                            match uistate.active_request_tab() {
                                RequestTabs::UrlParams => {
                                    let params = uistate.url_params_mut();
                                    let apr = params.active_param_col();

                                    if apr > 0 {
                                        params.set_active_param_col(apr - 1);
                                    }
                                },
                                _ => {},
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
                                let mut apr = uistate.url_params_mut()
                                    .active_param_row();

                                if apr <= 1000 {
                                    apr += 1;
                                    uistate.url_params_mut().set_active_param_row(apr);
                                }
                            },
                            _ => {},
                        }
                    }
                },
                _ => {  },
            };

            if url_params_changed {
                let updated_url = url_with_params(
                        uistate.url(),
                        uistate.url_params().params().to_vec()
                    );

                if !updated_url.is_empty() {
                    uistate.set_url(updated_url);
                }
            }
        }
    }

    Ok(false)
}

