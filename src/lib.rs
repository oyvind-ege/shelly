mod commands;
use util::parse_args_from_str_with_quotes;

use crate::commands::*;
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
pub struct Shell {}

impl Shell {
    pub fn initiate(input: String) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        let (command, args) = parse_command_and_arguments(&input);
        let valid_commands = get_executables_from_paths(get_paths()).unwrap_or_default();

        match command {
            cmd if !BUILTINS.contains(&cmd) && !valid_commands.contains_key(cmd) => {
                Ok(Box::new(InvalidCommand::new(cmd.to_string())))
            }
            "exit" => Ok(Box::new(ExitCommand::new(args.clone()))),
            "echo" => Ok(Box::new(EchoCommand::new(args.clone()))),
            "type" => Ok(Box::new(TypeCommand::new(args.clone(), valid_commands))),
            "pwd" => Ok(Box::new(PwdCommand::new())),
            "cd" => Ok(Box::new(CdCommand::new(args.clone()))),

            cmd if valid_commands.contains_key(cmd) => Ok(Box::new(RunCommand::new(
                args.clone(),
                get_command_info(&valid_commands, cmd),
            ))),

            _ => Err("Try again.".into()),
        }
    }
}
pub fn get_executables_from_paths(pbs: Vec<PathBuf>) -> io::Result<HashMap<String, OsString>> {
    let mut executables: HashMap<String, OsString> = HashMap::new();
    for dir in pbs {
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

                    executables
                        .entry(binary_name)
                        .or_insert(path.clone().into_os_string());
                }
            }
        }
    }
    Ok(executables)
}

pub fn get_paths() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(v) => env::split_paths(&v).collect(),

        None => todo!(),
    }
}

pub fn parse_command_and_arguments(input: &str) -> (&str, Vec<String>) {
    let split_input = input.splitn(2, ' ').collect::<Vec<&str>>();
    let args = if split_input.len() > 1 {
        parse_args_from_str_with_quotes(split_input[1])
    } else {
        vec![]
    };

    let command = split_input[0];

    (command, args)
}

pub fn get_command_info(valid_commands: &HashMap<String, OsString>, command: &str) -> CommandInfo {
    let command_borrowed = valid_commands.get_key_value(command).unwrap();

    CommandInfo {
        bin: command_borrowed.0.to_string(),
        path: command_borrowed.1.to_owned(),
    }
}

#[cfg(test)]
mod tests {

    use super::parse_command_and_arguments;
    #[test]
    fn test_parse_command() {
        let input = String::from("cmd x y z");
        let cmd = "cmd";
        let args = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn test_empty_args() {
        let input = String::from("cmd");
        let cmd = "cmd";
        let args: Vec<String> = vec![];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn test_whitespace() {
        let input = String::from("cmd   ");
        let cmd = "cmd";
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }
}
