const EMPTY_STRING: String = String::new();

#[derive(Clone, Default, PartialEq)]
pub struct KVData {
    key: String,
    value: String,
}

impl KVData {
    pub const fn default_const() -> Self {
        Self { key: EMPTY_STRING, value: EMPTY_STRING }
    }

    pub fn new(key: String, value: String) -> Self { KVData { key, value } }

    pub fn from_string(input: String) -> Self { Self::from_str(&input) }

    pub fn from_str(input: &str) -> Self {
        if input.is_empty() {
            return KVData::default();
        }

        let inp_vec: Vec<&str> = input.split('=').collect();

        let key = String::from(inp_vec[0]);
        let mut value = String::default();

        if inp_vec.len() == 2 {
            value.push_str(inp_vec[1]);
        }

        KVData { key, value }
    }

    pub fn to_string(&self) -> String {
        if !self.key.is_empty() && !self.value.is_empty() {
            return format!("{}={}", self.key, self.value);
        }

        if !self.key.is_empty() {
            return self.key();
        }

        String::from("")
    }

    pub fn key(&self) -> String { self.key.clone() }
    pub fn set_key(&mut self, key: String) { self.key = key; }

    pub fn value(&self) -> String { self.value.clone() }
    pub fn set_value(&mut self, value: String) { self.value = value; }
}

