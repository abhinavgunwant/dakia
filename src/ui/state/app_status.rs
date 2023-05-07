#[derive(Clone)]
pub enum AppStatus {
    /// Denotes the startup state - no request have been made
    STARTUP,
    PROCESSING,
    DONE,
    ERROR,
}

impl Default for AppStatus {
    fn default() -> Self { Self::STARTUP }
}

impl AppStatus {
    pub fn to_str(&self) -> &str {
        match self {
            Self::STARTUP => "",
            Self::PROCESSING => "Processing",
            Self::DONE => "Done",
            Self::ERROR => "Error",
        }
    }
}

