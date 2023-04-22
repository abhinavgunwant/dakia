use std::io::Error;

use crossterm::event::{ self, Event, KeyCode };
use crate::{
    ui::state::{ UiState, InputMode, EditorMode, UIElement, Method },
    api::call_api,
};

/**
 * User interaction related code...
 *
 * Returning Ok(true) shall exit the program.
 */
pub fn process_user_input(uistate: &mut UiState) -> Result<bool, Error> {
    if uistate.clone().editor_mode() == EditorMode::Normal
        || uistate.clone().input_mode() == InputMode::Normal
    {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    if *uistate.active_element() == UIElement::RequestTabsElem
                        && uistate.inside_request_tabs() {
                            uistate.set_inside_request_tabs(false);
                    } else {
                        return Ok(true);
                    }
                },
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
                        _ => {},
                    }
                },
                KeyCode::Backspace => { uistate.pop_url(); },
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
                            uistate.set_inside_request_tabs(true);
                        },
                        _ => {},
                    }
                }
                KeyCode::Tab => {
                    if *uistate.active_element() == UIElement::RequestTabsElem
                        && uistate.inside_request_tabs()
                    {
                        uistate.activate_next_req_tab();
                    } else {
                        uistate.activate_next_element();
                    }
                },
                KeyCode::BackTab => {
                    if *uistate.active_element() == UIElement::RequestTabsElem
                        && uistate.inside_request_tabs()
                    {
                        uistate.activate_previous_req_tab();
                    } else {
                        uistate.activate_previous_element();
                    }
                },
                _ => {  },
            };
        }
    }

    Ok(false)
}

