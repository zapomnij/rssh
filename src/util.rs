pub struct Config {
    pub error_ex: bool,
    pub prompt: String,
    pub run_script: bool,
    pub scripts: Vec<String>,
    pub cmd: String,
}

use std::{env, path::Path, process::exit};

extern crate getopts;
use getopts::Options;

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let mut run_script: bool = true;
        if args.len() < 2 {
            run_script = false;
        }

        let mut opts = Options::new();
        opts.optflag("e", "error", "Exit on error");
        opts.optopt("c", "command", "Execute command", "");
        opts.optflag("h", "help", "Display help message");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                eprintln!("rssh: cannot parse options: {}", f);
                exit(1);
            }
        };

        if matches.opt_present("h") {
            println!("rssh - simple unix shell written in rust");
            println!(
                "Usage: {} ( -h ) [ -c command ] [ -e ] [ scripts ]\n",
                args[0]
            );
            println!("Options:");
            println!("  -h, --help              -> Display help message");
            println!("  -c, --command 'command' -> Execute command from shell argument");
            println!(
                "  -e, --error             -> Shell will be terminated, if command gets error"
            );
            exit(0);
        }

        let mut error_ex: bool = false;
        if matches.opt_present("e") {
            error_ex = true;
        }

        let mut cmd = String::new();
        if matches.opt_present("c") {
            cmd = match matches.opt_str("c") {
                Some(t) => t,
                None => {
                    eprintln!("rssh: missing argument");
                    exit(1);
                }
            };
        }

        let mut scripts = Vec::new();
        for arg in matches.free {
            scripts.push(arg);
        }

        let prompt: String = match env::var("PROMPT") {
            Err(_) => String::from("$ "),
            Ok(s) => s,
        };

        Config {
            error_ex,
            prompt,
            run_script,
            scripts,
            cmd,
        }
    }
}

pub fn parsecmd(line: &String) -> Vec<&str> {
    let cmd = line.split_whitespace().collect();
    cmd
}

use std::fs;

pub fn parsescript(path: &String, config: &Config) {
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

        let res = execcmd(parsecmd(&i.to_string()), &config);
        if config.error_ex == true {
            if !res.unwrap().success() {
                exit(1);
            }
        }
    }
}

use std::process::Command;

pub fn execcmd(cmd_buf: Vec<&str>, config: &Config) -> std::io::Result<std::process::ExitStatus> {
    let mut cmd_e: Vec<String> = Vec::new();

    for &i in &cmd_buf {
        if i.chars().nth(0).unwrap() == '$' {
            let addr = &i[1..];
            let buf = match env::var(addr) {
                Err(_) => String::from(""),
                Ok(o) => o,
            };

            cmd_e.push(buf.clone());
            break;
        }

        cmd_e.push(i.to_string());
    }

    if cmd_e[0].clone().eq(".") {
        if cmd_e.len() < 2 {
            eprintln!("rssh: missing argument");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }

        parsescript(&cmd_e[1], &config);
        return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
    }

    if cmd_e[0].clone().eq("export") {
        if cmd_e.len() < 2 {
            eprintln!("rssh: missing argument");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }

        let split: Vec<&str> = cmd_e[1].split("=").collect();

        if split.len() < 2 {
            eprintln!("rssh: passed argument is not valid environment variable");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }

        env::set_var(split[0].clone(), split[1].clone());
        return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
    }

    if cmd_e[0].clone().eq("getenv") {
        if cmd_e.len() < 2 {
            eprintln!("rssh: missing argument");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }
        match env::var(&cmd_e[1]) {
            Err(e) => {
                eprintln!("rssh: failed to get environment variable. {e}");
                return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
            }
            Ok(o) => {
                println!("{o}");
                return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
            }
        }
    }

    if cmd_e[0].clone().eq("unset") {
        if cmd_e.len() < 2 {
            eprintln!("rssh: missing argument");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }

        env::remove_var(&cmd_e[1]);
        return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
    }

    if cmd_e[0].clone().eq("cd") {
        match env::set_current_dir(Path::new(&cmd_e[1])) {
            Err(e) => {
                eprintln!("rssh: failed to change directory to '{}'. {}", &cmd_e[1], e);
                return Err(e);
            }
            Ok(_) => {
                return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
            }
        };
    }

    if cmd_e[0].clone().eq("exit") {
        if cmd_e.len() > 1 {
            exit(match cmd_e[1].trim().parse() {
                Err(_) => {
                    eprintln!("rssh: exit: please use 32 bit signed integer as the exit value. Other values are not supported");
                    255
                }
                Ok(v) => v,
            });
        }
        exit(0);
    }

    let mut cmd = Command::new(&cmd_e[0]);
    cmd.args(&cmd_e[1..]);

    cmd.status()
}
