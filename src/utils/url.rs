//! URL related utilities

use crate::ui::state::url_params::Param;

pub fn url_with_params(url: String, params: Vec<Param>) -> String {
    if url.is_empty() {
        return String::default();
    }

    let mut new_url: String = url.clone();

    let mut params_appended_to_url: bool = false;
    let params = params;

    for (_i, p) in params.iter().enumerate() {
        let p_name = p.clone().name();
        let p_value = p.clone().value();

        if !p_name.is_empty() && !p_value.is_empty() {
            if params_appended_to_url {
                new_url.push('&');
            }

            if !params_appended_to_url {
                new_url.push('?');
                params_appended_to_url = true;
            }

            new_url.push_str(p_name.as_str());
            new_url.push('=');
            new_url.push_str(p_value.as_str());
        }
    }

    new_url
}

