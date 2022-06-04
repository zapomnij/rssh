use crate::util;

pub fn parsecmd(line: &String) -> Vec<&str> {
    let dquoteparse: Vec<&str> = line.split("\"").collect();

    let mut dquoteparsed: Vec<&str> = Vec::new();
    let mut index: usize = 1;
    for parse in dquoteparse {
        if (index % 2) == 0 {
            dquoteparsed.push(&parse);
        } else {
            let split: Vec<&str> = parse.split_whitespace().collect();
            for i in split {
                dquoteparsed.push(i);
            }
        }
        index += 1;
    }

    dquoteparsed
}

use std::fs;
use std::process::exit;

pub mod execcmd;

pub fn parsescript(path: &String, config: &util::Config) {
    let content: String = match fs::read_to_string(&path) {
        Err(e) => {
            eprintln!("rssh: failed to read from {path}. {e}");
            if config.error_ex == true {
                exit(255);
            }
            return;
        }
        Ok(o) => o,
    };

    for i in content.lines() {
        if i.len() < 1 {
            continue;
        }
        if i.chars().nth(0).unwrap() == '#' {
            continue;
        }

        let res = execcmd::execcmd(parsecmd(&i.to_string()), &config);
        if config.error_ex == true {
            if !res.unwrap().success() {
                exit(1);
            }
        }
    }
}
