use std::fmt::{Display, Formatter, Result};
use crate::ui::state::kv_data::KVData;

/// Deconstructs url
#[derive(Clone)]
pub struct Url {
    protocol: Protocol,
    host: String,
    port: u16,
    path: String,
    query_params: Vec<KVData>,
}

#[derive(Clone, PartialEq, Eq)]
enum Protocol { HTTP, HTTPS }

impl Protocol {
    pub fn to_str(&self) -> &str {
        match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

impl Default for Url {
    fn default() -> Self {
        Self {
            protocol: Protocol::default(),
            host: String::default(),
            port: 80,
            path: String::default(),
            query_params: vec! [ KVData::default() ]
        }
    }
}

impl Url {
    /// Parse and update the url object from the supplied url string.
    ///
    /// **TODO** Modify to show url errors.
    pub fn update(&mut self, url: String) {
        let mut url_contains_protocol = true;
        let mut url_contains_port = true;

        let url_temp = url.replace("://", ":");

        let url_tokens: Vec<&str> = url_temp.split(&[':', '?']).collect();
        // Would only have following scenarios that are valid URLs as a result
        // of this split:
        // 1. Length = 1
        //     -> example.com
        //     -> example.com/some/thing
        //
        // 2. Length = 2
        //     --> example.com:8080
        //         -> example.com, 8080
        //     --> example.com:3000/some/thing
        //         -> example.com, 3000/some/thing
        //     --> http://example.com
        //         -> http, example.com
        // 
        // 3. Length = 3
        //     --> http://example.com:8080
        //         -> http, example.com, 8080
        //     --> example.com:8080/some/thing?some_thing=else
        //         -> example.com, 8080/some/thing, some_thing=else
        //
        // 4. Length = 4
        //     --> http://localhost:8080/api?something=else&or_maybe=something
        //         -> http, localhost, 8080/api, something=else&or_maybe=something

        let len = url_tokens.len();

        if len == 1 {
            self.protocol = Protocol::HTTP;
            self.host = url.clone();
            self.port = 80;

            return;
        }

        let mut tok_indx = 1;

        // By default, also assign port
        match url_tokens[0] {
            "http" => {
                self.protocol = Protocol::HTTP;
                self.port = 80;
            },

            "https" => {
                self.protocol = Protocol::HTTPS;
                self.port = 443;
            },

            _ => {
                self.protocol = Protocol::HTTP;
                url_contains_protocol = false;
            }
        }
        
        if url_contains_protocol {
            self.host = String::from(url_tokens[1]);
            tok_indx = 2;

            // this case would completely match `http://example.com` so exit
            // if token length is == 2
            if len == 2 {
                return;
            }
        } else {
            self.host = String::from(url_tokens[0]);
            tok_indx = 1;
        }

        // We've got protocol and host, now we need to check if next token
        // contains just the port or port AND path.
        match url_tokens[tok_indx].parse::<u16>() {
            Ok (port_num) => {
                self.port = port_num;
                tok_indx += 1;
            },

            Err (_) => {
                // Here we need to check if next token contains port and path
                let str_to_check = url_tokens[tok_indx];
                let mut has_port = false;
                let mut port_indx_end = 0;

                for c in str_to_check.chars() {
                    if c.is_numeric() {
                        has_port = true;
                        port_indx_end += 1;
                    } else {
                        break;
                    }
                }

                if has_port {
                    let port_str = &str_to_check[0..port_indx_end];

                    match port_str.parse::<u16>() {
                        Ok (port) => {
                            self.port = port;

                            self.path = String::from(
                                str_to_check.replace(port_str, "")
                            );

                            tok_indx += 1;
                        },

                        Err(_) => {
                            self.path = String::from(str_to_check);
                            tok_indx += 1;
                        },
                    }

                    // now we need to check if this string has path
                }
            }
        }

        if tok_indx == len {
            return;
        }

        self.query_params_from_str(url_tokens[tok_indx]);
    }

    pub fn to_string(&self) -> String {
        let mut url_string = String::new();
        url_string.push_str(self.protocol.to_str());
        url_string.push_str("://");
        url_string.push_str(&self.host);

        if self.protocol == Protocol::HTTP && self.port != 80
            || self.protocol == Protocol::HTTPS && self.port != 443
        {
            url_string.push(':');
            url_string.push_str(&self.port.to_string());
        }

        if !self.path.is_empty() {
            url_string.push_str(&self.path);
        }

        if !self.query_params.is_empty() {
            url_string.push('?');
            url_string.push_str(&self.query_params_to_string());
        }

        url_string
    }

    fn query_params_from_str(&mut self, qp_str: &str) {
        let qp_split: Vec<&str> = qp_str.split('&').collect();

        self.query_params = vec![];

        for qp_str_pair in qp_split.iter() {
            if !qp_str_pair.is_empty() {
                self.query_params.push(KVData::from_str(qp_str_pair));
            }
        }

        if self.query_params.is_empty() {
            self.query_params.push(KVData::default());
        }
    }

    fn query_params_to_string(&self) -> String {
        let mut query_string = String::default();

        for (i, query_param) in self.query_params.iter().enumerate() {
            let qstr = &query_param.to_string();

            if !qstr.is_empty() {
                if i > 0 {
                    query_string.push('&');
                }

                query_string.push_str(qstr);
            }
        }

        query_string
    }

    pub fn protocol(&self) -> Protocol { self.protocol.clone() }
    pub fn host(&self) -> String { self.host.clone() }
    pub fn port(&self) -> u16 { self.port }
    pub fn path(&self) -> String { self.path.clone() }
    pub fn query_params(&self) -> &Vec<KVData> { &self.query_params }

    pub fn get_param(&mut self, indx: u16) -> Option<&mut KVData> {
        self.query_params.get_mut(indx as usize)
    }
    /// Inserts a new param to the `pos` position in `params`.
    /// Param limit is set to `1000`.
    pub fn insert_param(&mut self, pos: u16, param: KVData) {
        if self.query_params.len() == 1000 || pos == 1000{
            return;
        }

        self.query_params.insert(pos as usize, param);
    }
    /// Removes param in `pos` position.
    pub fn remove_param(&mut self, pos: u16) {
        self.query_params.remove(pos as usize);

        if self.query_params.len() == 0 {
            self.query_params.push(KVData::default());
        }
    }
}

