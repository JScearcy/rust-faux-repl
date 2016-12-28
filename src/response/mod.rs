use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    #[serde(rename="program")]
    pub output: Option<String>,
    #[serde(default)]
    pub rustc: String,
    #[serde(default)]
    pub error: Option<String>,
}

impl Default for Response {
    fn default() -> Response {
        Response {
            output: None,
            rustc: String::new(),
            error: None,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.output {
            Some(ref out) => write!(f, "{}", out),
            None => write!(f, "{}", self.rustc),
        }
    }
}
