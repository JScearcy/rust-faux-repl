#![feature(custom_attribute, custom_derive, proc_macro)]
extern crate regex;
extern crate serde;
extern crate serde_json as json;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

mod request;
mod response;
mod args;
mod commands;
mod code;

use std::io::{stdout, stdin, Write};
use std::env;
use code::Code;

fn main() {
    let action: bool = args::args_handler(env::args());
    if action != true {
        let mut quit = false;
        let mut code = Code::new();
        loop {
            if quit {
                break;
            }
            let mut input = String::new();
            let writes = stdout()
                .write(b"> ")
                .and(stdout().flush())
                .and(stdin().read_line(&mut input));

            match writes {
                Ok(_) => {
                    quit = commands::handle_commands(input, &mut code);
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }
}
