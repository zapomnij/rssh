pub mod util;

use std::io::Write;
use std::process::exit;

fn prompt(config: &util::Config) {
    print!("{}", config.prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(e) => {
            eprintln!("rssh: failed to read from stdin {}", e);
            exit(1);
        }
        Ok(ok) => ok,
    };

    if input.len() == 1 {
        return;
    }

    let parsed = util::parsecmd(&input);
    let ret = util::execcmd(parsed);
    match ret {
        Err(e) => {
            eprintln!("rssh: failed to execute command. {}", e);
            if config.error_ex == true {
                exit(255);
            }
        }
        Ok(num) => {
            if config.error_ex == true {
                if !num.success() {
                    exit(1);
                }
            }
        }
    }
}

use std::fs;

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

        let res = util::execcmd(util::parsecmd(&i.to_string()));
        if config.error_ex == true {
            if !res.unwrap().success() {
                exit(1);
            }
        }
    }
}

fn main() {
    let config = util::Config::new();

    if config.scripts.len() > 0 {
        for i in &config.scripts {
            parsescript(&i, &config);
        }
        exit(0);
    }

    loop {
        prompt(&config);
    }
}
