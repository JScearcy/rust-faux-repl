#[derive(Serialize)]
pub struct Request<'a> {
    pub code: &'a str,
    pub version: &'a str,
    pub optimize: &'a str,
    pub backtrace: &'a str,
    pub color: bool,
    pub test: bool,
    separate_output: bool,
}

impl<'a> Default for Request<'a> {
    fn default() -> Request<'a> {
        Request {
            code: "",
            version: "nightly",
            optimize: "2",
            backtrace: "1",
            color: false,
            test: false,
            separate_output: true,
        }
    }
}

impl<'a> From<&'a str> for Request<'a> {
    fn from(s: &str) -> Request {
        Request { code: s, ..Default::default() }
    }
}
