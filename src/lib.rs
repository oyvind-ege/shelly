mod commands;
mod parse;
use crate::parse::*;

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
        let valid_commands: HashMap<String, OsString> =
            get_binaries_from_paths(get_path_variable()).unwrap_or_default();

        match command.as_str() {
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

pub fn parse_command_and_arguments(input: &str) -> (String, Vec<String>) {
    let parsed = parse_input(input);

    let cmd = parsed.cmd.unwrap_or_default();

    let args = parsed.args.unwrap_or_default();

    (cmd, args)
}

pub fn get_command_info(valid_commands: &HashMap<String, OsString>, command: &str) -> CommandInfo {
    let command_borrowed = valid_commands.get_key_value(command).unwrap();

    CommandInfo {
        bin: command_borrowed.0.to_string(),
        path: command_borrowed.1.to_owned(),
    }
}

#[cfg(test)]
mod parse_commands_test {

    use super::parse_command_and_arguments;
    #[test]
    fn simple_command_with_three_arguments() {
        let input = String::from("cmd x y z");
        let cmd = "cmd".to_string();
        let args = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn empty() {
        let input = String::from("");
        let expected = ("".to_string(), Vec::<String>::new());
        let result = parse_command_and_arguments(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn no_arguments() {
        let input = String::from("cmd");
        let cmd = "cmd".to_string();
        let args: Vec<String> = vec![];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn whitespace() {
        let input = String::from("cmd   ");
        let cmd = "cmd".to_string();
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn single_quotes_around_command() {
        let input = String::from("'cmd'");
        let cmd = "cmd".to_string();
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn double_quote_around_command() {
        let input = String::from(r#""cmd""#);
        let cmd = "cmd".to_string();
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn command_with_space() {
        let input = String::from(r#"'cmd with space'"#);
        let cmd = "cmd with space".to_string();
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn command_with_double_quotes_and_space() {
        let input = String::from(r#""cmd with space""#);
        let cmd = "cmd with space".to_string();
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }
}

#[cfg(test)]
mod output_redirection {
    use super::*;

    #[test]
    fn output_file_identified() {
        let input = String::from(r#"echo hello > hello.txt"#);
        let args: Vec<String> = vec![];
    }
}
