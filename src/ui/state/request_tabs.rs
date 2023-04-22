use std::iter::Iterator;

#[derive(Clone, Copy, PartialEq)]
pub enum RequestTabs {
    UrlParams = 0,
    Authorization = 1,
    Headers = 2,
    Body = 3,
}

impl Default for RequestTabs {
    fn default() -> Self { RequestTabs::UrlParams }
}

impl RequestTabs {
    pub fn get_str_label(self) -> String {
        match self {
            RequestTabs::UrlParams => String::from("URL Params"),
            RequestTabs::Authorization => String::from("Authorization"),
            RequestTabs::Headers => String::from("Headers"),
            RequestTabs::Body => String::from("Body"),
        }
    }

    pub fn get_val(&self) -> u8 { *self as u8 }

    pub fn from_val(val: u8) -> Self {
        match val {
            0 => RequestTabs::UrlParams,
            1 => RequestTabs::Authorization,
            2 => RequestTabs::Headers,
            3 => RequestTabs::Body,
            _ => RequestTabs::UrlParams,
        }
    }

    pub fn iter() -> impl Iterator<Item = RequestTabs> {
        [
            Self::UrlParams, Self::Authorization, Self::Headers, Self::Body,
        ].iter().copied()
    }
}

