use std::{
    error::Error, time::Duration, str::FromStr,
};
use reqwest::{
    blocking::{ Response, Client, ClientBuilder },
    header::{ self, HeaderMap, HeaderValue, HeaderName }, Method,
};

use jsonxf::pretty_print;
use crate::ui::state::{UiState, app_status::AppStatus};

const APP_JSON: &str = "application/json";
const TEXT_HTML: &str = "text/html";
const APP_XHTML_XML: &str = "application/xhtml+xml";
const APP_XML: &str = "application/xml";

pub fn call_api(uistate: &mut UiState) -> Result<(), Box<dyn Error + 'static>> {
    let mut def_headers: HeaderMap = HeaderMap::new();
    let accept_val = format!("{},{},{}", TEXT_HTML, APP_XHTML_XML, APP_XML);

    def_headers.insert("accept", HeaderValue::from_str(accept_val.as_str()).unwrap());
    def_headers.insert("accept-encoding", HeaderValue::from_str("gzip, deflate, br").unwrap());
    def_headers.insert("accept-language", HeaderValue::from_str("en-US,en;").unwrap());
    def_headers.insert("cache-control", HeaderValue::from_str("no-cache").unwrap());
    def_headers.insert("pragma", HeaderValue::from_str("no-cache").unwrap());

    let client: Client = ClientBuilder::new()
        .timeout(Duration::from_secs(20))
        .user_agent("Dakia/0.1.0")
        .default_headers(def_headers)
        .build()?;


    let mut request = client.request(
        uistate.method(),
        uistate.url_deconst().to_string()
    );

    if !uistate.request_headers().is_empty() {
        let mut headers: HeaderMap = HeaderMap::new();

        for header in uistate.request_headers().iter() {
            if !header.key().is_empty() && !header.value().is_empty() {
                headers.insert(
                    HeaderName::from_str(header.key().as_str()).unwrap(),
                    HeaderValue::from_str(header.value().as_str()).unwrap()
                );
            }
        }

        request = request.headers(headers);
    }

    let response = request.send()?;
//    let response = client.get(uistate.url_deconst().to_string()).send()?;
    uistate.increment_request_counter();
    uistate.set_response_status_code(Some(response.status().as_u16()));

    process_response(response, uistate);

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

        is_json = hdr.starts_with(APP_JSON);
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

