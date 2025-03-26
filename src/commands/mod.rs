use crate::CommandInfo;
use crate::CommandOptions;

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
    options: CommandOptions,
}

#[derive(Debug)]
pub struct EchoCommand {
    options: CommandOptions,
}

#[derive(Debug)]
pub struct TypeCommand<'a> {
    options: CommandOptions,
    valid_commands: &'a HashMap<String, OsString>,
}

#[derive(Debug)]
pub struct InvalidCommand {
    options: CommandOptions,
}

#[derive(Debug)]
pub struct RunCommand {
    options: CommandOptions,
    command: CommandInfo,
}

#[derive(Debug)]
pub struct PwdCommand {}

#[derive(Debug)]
pub struct CdCommand {
    options: CommandOptions,
}

/*******************************
 ------------ Exit ------------
*******************************/

impl ExitCommand {
    pub fn new(options: CommandOptions) -> Self {
        ExitCommand { options }
    }
}

impl Execute for ExitCommand {
    fn execute(&self) {
        match &self.options.args {
            Some(val) if val.first().unwrap() == "0" => process::exit(exitcode::OK),
            _ => process::exit(exitcode::USAGE),
        }
    }
}

/*******************************
 ------------ Echo ------------
*******************************/
impl EchoCommand {
    pub fn new(options: CommandOptions) -> Self {
        EchoCommand { options }
    }
}

impl Execute for EchoCommand {
    fn execute(&self) {
        println!("{}", self.options.args.clone().unwrap().join(" "));
    }
}

/*******************************
 ------------ Invalid ------------
*******************************/
impl InvalidCommand {
    pub fn new(options: CommandOptions) -> Self {
        InvalidCommand { options }
    }
}

impl Execute for InvalidCommand {
    fn execute(&self) {
        println!("{}: command not found", self.options.cmd.clone().unwrap());
    }
}

/*******************************
 ------------ Run ------------
*******************************/
impl RunCommand {
    pub fn new(options: CommandOptions, command: CommandInfo) -> Self {
        RunCommand { command, options }
    }
}

impl Execute for RunCommand {
    fn execute(&self) {
        let output = Command::new(self.command.bin.clone())
            .args(self.options.args.clone().unwrap())
            .output()
            .expect("Failed to run command.");

        io::stdout().write_all(&output.stdout).unwrap();
    }
}

/*******************************
 ------------ Type ------------
*******************************/
impl<'a> TypeCommand<'a> {
    pub fn new(options: CommandOptions, valid_commands: &'a HashMap<String, OsString>) -> Self {
        TypeCommand {
            options,
            valid_commands,
        }
    }
}

impl Execute for TypeCommand<'_> {
    fn execute(&self) {
        match &self.options.args.clone().unwrap().first() {
            Some(bin)
                if !BUILTINS.contains(&bin.as_str()) && !self.valid_commands.contains_key(*bin) =>
            {
                println!("{}: not found", bin)
            }
            Some(bin) if BUILTINS.contains(&bin.as_str()) => println!("{} is a shell builtin", bin),
            Some(bin) if self.valid_commands.contains_key(*bin) => {
                println!(
                    "{} is {}",
                    bin,
                    self.valid_commands.get(*bin).unwrap().to_str().unwrap()
                )
            }
            None => println!("Wrong usage"), //this right here is the entry point for a manpage message
            Some(_) => println!(),
        };
    }
}

/*******************************
 ------------ PWD ------------
*******************************/
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

/*******************************
 ------------ Cd ------------
*******************************/
impl CdCommand {
    pub fn new(options: CommandOptions) -> Self {
        CdCommand { options }
    }
}

impl Execute for CdCommand {
    fn execute(&self) {
        if let Some(path) = &self.options.args.clone().unwrap().first() {
            match path {
                path if *path == &"~".to_string() => {
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
