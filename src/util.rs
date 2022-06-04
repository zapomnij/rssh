pub struct Config {
    pub error_ex: bool,
    pub prompt: String,
    pub run_script: bool,
    pub scripts: Vec<String>,
    pub cmd: String,
}

use std::{env, process::exit};

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
