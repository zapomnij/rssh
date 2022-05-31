pub struct Config {
    pub error_ex: bool,
    pub prompt: String,
    pub run_script: bool,
    pub scripts: Vec<String>,
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
        opts.optflag("e", "--error", "Exit on error");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                eprintln!("rssh: cannot parse options: {}", f);
                exit(1);
            }
        };

        let mut error_ex: bool = false;
        if matches.opt_present("e") {
            error_ex = true;
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
        }
    }
}

pub fn parsecmd(line: &String) -> Vec<&str> {
    let cmd = line.split_whitespace().collect();
    cmd
}

pub fn execcmd(cmd_e: Vec<&str>) -> std::io::Result<std::process::ExitStatus> {
    if cmd_e[0].clone().eq("export") {
        if cmd_e.len() < 2 {
            eprintln!("rssh: missing argument");
            return Ok(std::os::unix::process::ExitStatusExt::from_raw(1));
        }

        let split: Vec<&str> = cmd_e[1].clone().split("=").collect();

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

    let mut cmd = std::process::Command::new(&cmd_e[0]);
    cmd.args(&cmd_e[1..]);
    match cmd.output() {
        Err(e) => {
            eprintln!("rssh: failed to set output for command. {}", e);
            return Err(e);
        }
        Ok(o) => o,
    };

    cmd.status()
}
