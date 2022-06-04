pub mod execute;
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

    let parsed = execute::parsecmd(&input);
    let ret = execute::execcmd::execcmd(parsed, &config);
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

use std::path::Path;

fn main() {
    let mut config = util::Config::new();

    if config.cmd.len() > 0 {
        if !config.cmd.eq("") {
            let res = match execute::execcmd::execcmd(execute::parsecmd(&config.cmd), &config) {
                Err(e) => {
                    eprintln!("rssh: failed to execute command. {e}");
                    exit(1);
                }
                Ok(o) => o,
            };

            if !res.success() {
                exit(1);
            }
            exit(0);
        }
    }

    if config.scripts.len() > 0 {
        for i in &config.scripts {
            execute::parsescript(&i, &config);
        }
        exit(0);
    }

    let homedir = match std::env::var("HOME") {
        Err(_) => String::from("$ "),
        Ok(s) => s,
    };

    let format: String = format!("{}/.rssh_rc", homedir);
    let rc = Path::new(&format);
    if rc.is_file() {
        let format2: String = format!(". {}", &format);
        match execute::execcmd::execcmd(execute::parsecmd(&format2), &config) {
            Err(_) => {
                eprintln!("rssh: failed to source rssh_rc");
            }
            Ok(o) => {
                if !o.success() {
                    eprintln!("rssh: rssh_rc exited with non-zero status");
                }
            }
        };
    }

    loop {
        prompt(&config);
        config.prompt = match std::env::var("PROMPT") {
            Err(_) => String::from("$ "),
            Ok(s) => s,
        };
    }
}
