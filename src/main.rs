pub mod lib;

use std::io::Write;
use std::process::exit;

fn prompt(config: &lib::Config) {
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

    let parsed = lib::parsecmd(&input);
    let ret = lib::execcmd(parsed);
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

fn main() {
    let config = lib::Config::new();

    if config.scripts.len() > 0 {
        for i in &config.scripts {
            lib::parsescript(&i, &config);
        }
        exit(0);
    }

    loop {
        prompt(&config);
    }
}
