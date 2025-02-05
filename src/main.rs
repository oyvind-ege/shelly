use codecrafters_shell::{
    get_command_info, get_executables_from_paths, get_paths, parse_command_and_arguments,
    CommandInfo,
};
use core::error;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::process::{self};
extern crate exitcode;

static BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

trait Execute {
    fn execute(&self);
}

#[derive(Debug)]
struct Shell {}

#[derive(Debug)]
struct ExitCommand {
    args: Vec<String>,
}

#[derive(Debug)]
struct EchoCommand {
    args: Vec<String>,
}

#[derive(Debug)]
struct TypeCommand {
    args: Vec<String>,
    valid_commands: HashMap<String, OsString>,
}

#[derive(Debug)]
struct InvalidCommand {
    args: String,
}

#[derive(Debug)]
struct RunCommand {
    args: Vec<String>,
    command: CommandInfo,
}

#[derive(Debug)]
struct PwdCommand {}

#[derive(Debug)]
struct CdCommand {
    args: Vec<String>,
}

impl Shell {
    fn initiate(
        input: String,
        valid_external_commands: HashMap<String, OsString>,
    ) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        let (command, args) = parse_command_and_arguments(&input);

        match command {
            cmd if !BUILTINS.contains(&cmd) && !valid_external_commands.contains_key(cmd) => {
                Ok(Box::new(InvalidCommand {
                    args: cmd.to_string(),
                }))
            }
            "exit" => Ok(Box::new(ExitCommand { args: args.clone() })),
            "echo" => Ok(Box::new(EchoCommand { args: args.clone() })),
            "type" => Ok(Box::new(TypeCommand {
                args: args.clone(),
                valid_commands: valid_external_commands,
            })),
            "pwd" => Ok(Box::new(PwdCommand {})),
            "cd" => Ok(Box::new(CdCommand { args: args.clone() })),
            cmd if valid_external_commands.contains_key(cmd) => Ok(Box::new(RunCommand {
                args: args.clone(),
                command: get_command_info(&valid_external_commands, cmd),
            })),

            _ => todo!(),
        }
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

impl Execute for EchoCommand {
    fn execute(&self) {
        println!("{}", self.args.join(" "));
    }
}

impl Execute for InvalidCommand {
    fn execute(&self) {
        println!("{}: command not found", self.args);
    }
}

impl Execute for RunCommand {
    fn execute(&self) {
        let output = Command::new(self.command.bin.clone())
            .args(self.args.clone())
            .output()
            .expect("Failed");

        io::stdout().write_all(&output.stdout).unwrap();
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
            Some(_) => todo!(),
            None => println!("Wrong usage"), //this right here is the entry point for a manpage message
        };
    }
}

impl Execute for PwdCommand {
    fn execute(&self) {
        println!("{}", env::current_dir().expect("No current dir").display());
    }
}

impl Execute for CdCommand {
    fn execute(&self) {
        if let Some(path) = self.args.first() {
            match path {
                path if path == &"~".to_string() => {
                    env::set_current_dir(homedir::my_home().expect("fail").unwrap().as_path())
                        .expect("fail")
                }
                path => {
                    env::set_current_dir(Path::new(path)).expect("fail");
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let valid_commands = get_executables_from_paths(get_paths()).unwrap_or_default();
        Shell::initiate(input.trim().to_string(), valid_commands)?.execute();
    }
}
