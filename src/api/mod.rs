use std::error::Error;
use reqwest::blocking::get;
use crate::ui::state::UiState;

pub fn call_api(uistate: &mut UiState) -> Result<(), Box<dyn Error + 'static>> {
    match get(uistate.url()) {
        Ok (response) => {
            uistate.set_response_status_code(Some(response.status().as_u16()));

            match response.text() {
                Ok(response_text) => {
                    uistate.set_response(Some(response_text));
                },

                Err(_e) => {},
            }
        },

        Err(_e) => {},
    }

    Ok(())
}

