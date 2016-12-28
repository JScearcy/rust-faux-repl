use regex::Regex;
use std::env::Args;

pub fn args_handler(args: Args) -> bool {
    let mut cmds: Vec<String> = Vec::new();
    let mut quit: bool = false;
    let count_re = Regex::new(r"^(-count)(=?)(.*)$").unwrap();
    for argument in args.skip(1) {
        if argument == "-v" {
            cmds.push("Version 1.0.0".to_string());
            quit = true;
        } else if count_re.is_match(&argument) {
            let mut count_opt: Option<u16> = None;
            for cap in count_re.captures_iter(&argument) {
                count_opt = cap.at(3).map(|c| c.parse::<u16>().unwrap_or(0));
            }

            run_count(count_opt, &mut cmds);
            quit = true;
        } else {
            let err = format!("Error: {} command was not recognized", argument);
            cmds.push(err);
            quit = true;
        };
    }

    if cmds.len() > 0 {
        for cmd in cmds {
            println!("{}", cmd);
        }
    }

    quit
}

fn run_count(size_opt: Option<u16>, cmds: &mut Vec<String>) {
    let size = size_opt.unwrap_or(0);
    let mut count = 1;
    while count <= size {
        cmds.push(count.to_string());
        count = count + 1;
    }
}
