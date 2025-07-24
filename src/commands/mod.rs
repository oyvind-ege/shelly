use crate::CommandInfo;
use crate::ParsedCommand;
use std::io::Error;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::{self};

pub static BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub trait Execute {
    fn execute(&self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct ExitCommand {
    options: ParsedCommand,
}

#[derive(Debug)]
pub struct EchoCommand {
    options: ParsedCommand,
}

#[derive(Debug)]
pub struct TypeCommand {
    options: ParsedCommand,
    valid_commands: HashMap<String, OsString>,
}

#[derive(Debug)]
pub struct InvalidCommand {
    options: ParsedCommand,
}

#[derive(Debug)]
pub struct RunCommand {
    options: ParsedCommand,
    command: CommandInfo,
}

#[derive(Debug)]
pub struct PwdCommand {
    options: ParsedCommand,
}

#[derive(Debug)]
pub struct CdCommand {
    options: ParsedCommand,
}

/*******************************
 ------------ Exit ------------
*******************************/

impl ExitCommand {
    pub fn new(options: ParsedCommand) -> Self {
        ExitCommand { options }
    }
}

impl Execute for ExitCommand {
    fn execute(&self) -> Result<(), Error> {
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
    pub fn new(options: ParsedCommand) -> Self {
        EchoCommand { options }
    }
}

impl Execute for EchoCommand {
    fn execute(&self) -> Result<(), Error> {
        writeln!(
            self.options
                .get_output()
                .expect("Failed to get write output."),
            "{}",
            self.options.args.clone().unwrap().join(" ")
        )
    }
}

/*******************************
 ------------ Invalid ------------
*******************************/
impl InvalidCommand {
    pub fn new(options: ParsedCommand) -> Self {
        InvalidCommand { options }
    }
}

impl Execute for InvalidCommand {
    fn execute(&self) -> Result<(), Error> {
        writeln!(
            self.options
                .get_output()
                .expect("Failed to get write output."),
            "{}: command not found",
            self.options.cmd.clone().unwrap()
        )
    }
}

/*******************************
 ------------ Run ------------
*******************************/
impl RunCommand {
    pub fn new(options: ParsedCommand, command: CommandInfo) -> Self {
        RunCommand { command, options }
    }
}

impl Execute for RunCommand {
    fn execute(&self) -> Result<(), Error> {
        let mut out = self.options.get_output()?;
        let output = Command::new(self.command.bin.clone())
            .args(self.options.args.clone().unwrap_or_default())
            .output()
            .expect("Failed to run command.");

        out.write_all(&output.stdout)
    }
}

/*******************************
 ------------ Type ------------
*******************************/
impl TypeCommand {
    pub fn new(options: ParsedCommand, valid_commands: HashMap<String, OsString>) -> Self {
        TypeCommand {
            options,
            valid_commands,
        }
    }
}

impl Execute for TypeCommand {
    fn execute(&self) -> Result<(), Error> {
        let mut out = self
            .options
            .get_output()
            .expect("Failed to get write output.");
        match &self.options.args.clone().unwrap().first() {
            Some(bin)
                if !BUILTINS.contains(&bin.as_str()) && !self.valid_commands.contains_key(*bin) =>
            {
                writeln!(out, "{}: not found", bin)
            }
            Some(bin) if BUILTINS.contains(&bin.as_str()) => {
                writeln!(out, "{} is a shell builtin", bin)
            }

            Some(bin) if self.valid_commands.contains_key(*bin) => writeln!(
                out,
                "{} is {}",
                bin,
                self.valid_commands.get(*bin).unwrap().to_str().unwrap()
            ),
            None => writeln!(out, "Wrong usage"), //this right here is the entry point for a manpage message
            Some(_) => writeln!(out),
        }
    }
}

/*******************************
 ------------ PWD ------------
*******************************/
impl PwdCommand {
    pub fn new(options: ParsedCommand) -> Self {
        PwdCommand { options }
    }
}

impl Execute for PwdCommand {
    fn execute(&self) -> Result<(), Error> {
        writeln!(
            self.options.get_output().expect("Failed to get write"),
            "{}",
            env::current_dir().expect("No current dir").display()
        )
    }
}

/*******************************
 ------------ Cd ------------
*******************************/
impl CdCommand {
    pub fn new(options: ParsedCommand) -> Self {
        CdCommand { options }
    }
}

impl Execute for CdCommand {
    fn execute(&self) -> Result<(), Error> {
        let mut out = self.options.get_output().expect("Failed to get write.");
        if let Some(path) = &self.options.args.clone().unwrap().first() {
            match path {
                path if *path == &"~".to_string() => {
                    env::set_current_dir(env::var_os("HOME").unwrap_or_else(|| {
                        writeln!(out, "No HOME directory found.")
                            .expect("Failed to write to output.");
                        Path::new("").into()
                    }))?
                }
                path => {
                    env::set_current_dir(Path::new(path))
                        .unwrap_or_else(|_err| println!("cd: {}: No such file or directory", path));
                }
            }
        }
        Ok(())
    }
}
