use std::env;
use std::path::Path;
use std::process::{exit, Command};

use crate::util;

pub fn execcmd(
    cmd_buf: Vec<&str>,
    config: &util::Config,
) -> std::io::Result<std::process::ExitStatus> {
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

        crate::execute::parsescript(&cmd_e[1], &config);
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
