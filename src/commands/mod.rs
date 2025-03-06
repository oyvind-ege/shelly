use crate::CommandInfo;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::process::{self};

pub static BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub trait Execute {
    fn execute(&self);
}

#[derive(Debug)]
pub struct ExitCommand {
    args: Vec<String>,
}

#[derive(Debug)]
pub struct EchoCommand {
    args: Vec<String>,
}

#[derive(Debug)]
pub struct TypeCommand {
    args: Vec<String>,
    valid_commands: HashMap<String, OsString>,
}

#[derive(Debug)]
pub struct InvalidCommand {
    args: String,
}

#[derive(Debug)]
pub struct RunCommand {
    args: Vec<String>,
    command: CommandInfo,
}

#[derive(Debug)]
pub struct PwdCommand {}

#[derive(Debug)]
pub struct CdCommand {
    args: Vec<String>,
}

impl ExitCommand {
    pub fn new(args: Vec<String>) -> Self {
        ExitCommand { args }
    }
}

impl Execute for ExitCommand {
    fn execute(&self) {
        match self.args.first() {
            Some(val) if val == "0" => process::exit(exitcode::OK),
            _ => process::exit(exitcode::USAGE),
        }
    }
}

impl EchoCommand {
    pub fn new(args: Vec<String>) -> Self {
        EchoCommand { args }
    }
}

impl Execute for EchoCommand {
    fn execute(&self) {
        println!("{}", self.args.join(" "));
    }
}

impl InvalidCommand {
    pub fn new(args: String) -> Self {
        InvalidCommand { args }
    }
}

impl Execute for InvalidCommand {
    fn execute(&self) {
        println!("{}: command not found", self.args);
    }
}

impl RunCommand {
    pub fn new(args: Vec<String>, command: CommandInfo) -> Self {
        RunCommand { args, command }
    }
}

impl Execute for RunCommand {
    fn execute(&self) {
        let output = Command::new(self.command.bin.clone())
            .args(self.args.clone())
            .output()
            .expect("Failed to run command.");

        io::stdout().write_all(&output.stdout).unwrap();
    }
}

impl TypeCommand {
    pub fn new(args: Vec<String>, valid_commands: HashMap<String, OsString>) -> Self {
        TypeCommand {
            args,
            valid_commands,
        }
    }
}

impl Execute for TypeCommand {
    fn execute(&self) {
        match self.args.first() {
            Some(bin)
                if !BUILTINS.contains(&bin.as_str()) && !self.valid_commands.contains_key(bin) =>
            {
                println!("{}: not found", bin)
            }
            Some(bin) if BUILTINS.contains(&bin.as_str()) => println!("{} is a shell builtin", bin),
            Some(bin) if self.valid_commands.contains_key(bin) => {
                println!(
                    "{} is {}",
                    bin,
                    self.valid_commands.get(bin).unwrap().to_str().unwrap()
                )
            }
            None => println!("Wrong usage"), //this right here is the entry point for a manpage message
            Some(_) => println!(),
        };
    }
}

impl PwdCommand {
    pub fn new() -> Self {
        PwdCommand {}
    }
}

impl Execute for PwdCommand {
    fn execute(&self) {
        println!("{}", env::current_dir().expect("No current dir").display());
    }
}

impl CdCommand {
    pub fn new(args: Vec<String>) -> Self {
        CdCommand { args }
    }
}

impl Execute for CdCommand {
    fn execute(&self) {
        if let Some(path) = self.args.first() {
            match path {
                path if path == &"~".to_string() => {
                    env::set_current_dir(env::var_os("HOME").unwrap_or_else(|| {
                        println!("No HOME directory found.");
                        Path::new("").into()
                    }))
                    .expect("Failed to set current directory.")
                }
                path => {
                    env::set_current_dir(Path::new(path))
                        .unwrap_or_else(|_err| println!("cd: {}: No such file or directory", path));
                }
            }
        }
    }
}
