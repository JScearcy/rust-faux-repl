extern crate serde_json as json;
use reqwest;
use regex::Regex;
use std::io::Read;
use request::Request;
use response::Response;
use code::Code;

pub fn handle_commands(cmd: String, code: &mut Code) -> bool {
    let quit_re = Regex::new(r"^(quit|exit)\s*$").unwrap();
    let run_re = Regex::new(r"^(run)\s*$").unwrap();

    if quit_re.is_match(&cmd) {
        true
    } else if run_re.is_match(&cmd) {
        handle_code(code.get());

        code.clear();

        false
    } else {
        let newline_re = Regex::new(r"^[\r\n]+|\.|[\r\n]+$").unwrap();
        let mut new_code = cmd;
        new_code = newline_re.replace(&new_code, "");

        code.add(new_code);
        false
    }
}

fn handle_code(cmd: String) {
    let code = format!("fn main() {{ {} }}", cmd);
    let client = reqwest::Client::new().unwrap();
    let req = Request::from(&*code);

    let res: reqwest::Result<reqwest::Response> =
        client.post("https://play.rust-lang.org/evaluate.json")
            .json(&req)
            .send();

    let mut buffer = String::new();

    let success = match res {
        Ok(mut resp) => resp.read_to_string(&mut buffer).unwrap_or(usize::min_value()),
        Err(_) => usize::min_value(),
    };

    if success != usize::min_value() {
        let success_value: Response = json::from_str(&buffer).unwrap_or(Response::default());
        println!("{}", success_value);
    } else {
        println!("Res: {:?}", buffer);
    }
}
