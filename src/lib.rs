mod commands;
mod parse;
use crate::commands::*;
use crate::parse::*;
use std::collections::HashMap;
use std::env;
use std::error;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CommandInfo {
    pub bin: String,
    pub path: OsString,
}

#[derive(Debug)]
pub struct Shell {
    valid_commands: HashMap<String, OsString>,
}

impl Shell {
    pub fn init() -> Self {
        Shell {
            valid_commands: get_binaries_from_paths(get_path_variable()).unwrap_or_default(),
        }
    }

    pub fn parse(&self, input: String) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        let options = parse_input(&input);
        // We are now fetching these every input
        let valid_commands = get_binaries_from_paths(get_path_variable()).unwrap_or_default();
        match options.cmd.clone().unwrap_or_default().as_str() {
            cmd if !BUILTINS.contains(&cmd) && !self.valid_commands.contains_key(cmd) => {
                Ok(Box::new(InvalidCommand::new(options)))
            }
            "exit" => Ok(Box::new(ExitCommand::new(options))),
            "echo" => Ok(Box::new(EchoCommand::new(options))),
            "type" => Ok(Box::new(TypeCommand::new(options, valid_commands))),
            "pwd" => Ok(Box::new(PwdCommand::new(options))),
            "cd" => Ok(Box::new(CdCommand::new(options))),

            cmd if self.valid_commands.contains_key(cmd) => Ok(Box::new(RunCommand::new(
                options,
                get_command_info(&valid_commands, cmd),
            ))),

            _ => Err("Try again.".into()),
        }
    }
}

pub fn get_binaries_from_paths(paths: Vec<PathBuf>) -> io::Result<HashMap<String, OsString>> {
    let mut binaries: HashMap<String, OsString> = HashMap::new();
    for dir in paths {
        if dir.is_dir() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let binary_name = path
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap();

                    binaries
                        .entry(binary_name)
                        .or_insert(path.clone().into_os_string());
                }
            }
        }
    }
    Ok(binaries)
}

pub fn get_path_variable() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(v) => env::split_paths(&v).collect(),

        None => todo!(),
    }
}
/**
This function assumes that command is in valid_commands, and constructs a struct with the relevant information.
**/
pub fn get_command_info(valid_commands: &HashMap<String, OsString>, command: &str) -> CommandInfo {
    let command_borrowed = valid_commands.get_key_value(command).unwrap();

    CommandInfo {
        bin: command_borrowed.0.to_string(),
        path: command_borrowed.1.to_owned(),
    }
}
