#[derive(Clone, Default)]
pub struct Url {
    protocol: Protocol,
    host_name: String,
    port: u16,
    path: String,
    url_params: String,
}

#[derive(Clone)]
enum Protocol { HTTP, HTTPS }

impl Protocol {
    pub fn to_str(&self) -> &str {
        match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

