use std::error::Error;
use reqwest::{blocking::{ get, Response }, header};

use jsonxf::pretty_print;
use crate::ui::state::{UiState, app_status::AppStatus};

// CT: content-type
const CT_JSON: &str = "application/json";

pub fn call_api(uistate: &mut UiState) -> Result<(), Box<dyn Error + 'static>> {
    match get(uistate.url_deconst().to_string()) {
        Ok (response) => {
            uistate.increment_request_counter();
            uistate.set_response_status_code(Some(response.status().as_u16()));
            process_response(response, uistate);
        },

        Err(e) => {
            uistate.set_app_status(AppStatus::ERROR);
            uistate.set_app_error(e.without_url().to_string());
        },
    }

    Ok(())
}

fn process_response(resp: Response, uistate: &mut UiState) {
    let headers = resp.headers();
    let mut is_json = false;
    let mut error = false;

    if headers.contains_key(header::CONTENT_TYPE) {
        let hdr = String::from(
            headers[header::CONTENT_TYPE].to_str().unwrap()
        );

        is_json = hdr.starts_with(CT_JSON);
    }

    match resp.text() {
        Ok(response_text) => {
            let resp_state = uistate.response_mut();

            if is_json {
                match pretty_print(response_text.as_str()) {
                    Ok (pretty_json) => { resp_state.from_str(pretty_json); },
                    Err (just_json) => { resp_state.from_str(just_json); }
                }
            } else {
                resp_state.from_str(response_text);
            }
        },

        Err(_e) => { error = true; },
    }

    if error {
        uistate.set_app_status(AppStatus::ERROR);
    } else {
        uistate.set_app_status(AppStatus::DONE);
    }
}

