use codecrafters_shell::get_executables_from_paths;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self};
extern crate exitcode;

/*
TODO:

    - Move all logic into a separate lib
    - Implement PATH logic using std::env::var_os and split_path
*/

// This list is currently decoupled from the actual existence of command structs and implementations of Execute. Perhaps that is fine.
static COMMANDS: [&str; 3] = ["exit", "echo", "type"];

trait Execute {
    fn execute(&self);
}

#[derive(Debug)]
struct Command {
    args: Vec<String>,
}

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

impl Command {
    fn new() -> Self {
        Command { args: vec![] }
    }

    fn parse(&mut self, input: String) -> Result<Box<dyn Execute>, io::Error> {
        let split_input = input.split_whitespace().collect::<Vec<&str>>();

        // TODO: Consider moving parse() into command-specific implementation. That way we can sooner catch incorrect usage.
        self.args = if split_input.len() > 1 {
            split_input[1..]
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
        } else {
            [].to_vec()
        };

        let pbs = get_executables_from_paths(get_paths())?;

        match split_input[0] {
            cmd if !COMMANDS.contains(&cmd) => Ok(Box::new(InvalidCommand {
                args: cmd.to_string(),
            })),
            "exit" => Ok(Box::new(ExitCommand {
                args: self.args.clone(),
            })),
            "echo" => Ok(Box::new(EchoCommand {
                args: self.args.clone(),
            })),
            "type" => Ok(Box::new(TypeCommand {
                args: self.args.clone(),
                valid_commands: pbs,
            })),
            _ => todo!(),
        }
    }
}

fn get_paths() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(v) => env::split_paths(&v).collect(),
        None => todo!(),
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

impl Execute for TypeCommand {
    fn execute(&self) {
        match self.args.first() {
            Some(arg)
                if !COMMANDS.contains(&arg.as_str()) && !self.valid_commands.contains_key(arg) =>
            {
                println!("{}: not found", arg)
            }
            Some(arg) if COMMANDS.contains(&arg.as_str()) => println!("{} is a shell builtin", arg),
            Some(arg) if self.valid_commands.contains_key(arg) => {
                println!(
                    "{} is {}",
                    arg,
                    self.valid_commands.get(arg).unwrap().to_str().unwrap()
                )
            }
            Some(_) => todo!(),
            None => println!("Wrong usage"), //this right here is the entry point for a manpage message
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ENV is: {:?}", env::var_os("PATH").unwrap());
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        Command::new().parse(input.trim().to_string())?.execute();
    }
}
