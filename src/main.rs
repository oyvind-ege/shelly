use codecrafters_shell::{get_executables_from_paths, get_paths};
use core::error;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{self, Write};
use std::process::Command;
use std::process::{self};
extern crate exitcode;

static COMMANDS: [&str; 3] = ["exit", "echo", "type"];

trait Execute {
    fn execute(&self);
}

trait Parse {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>>;
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
    command: (String, OsString),
}

impl Shell {
    fn initiate(
        input: String,
        valid_external_commands: HashMap<String, OsString>,
    ) -> Result<Box<dyn Parse>, Box<dyn error::Error>> {
        let split_input = input.split_whitespace().collect::<Vec<&str>>();

        let args = if split_input.len() > 1 {
            split_input[1..]
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
        } else {
            [].to_vec()
        };

        let command = split_input[0];
        let mut command_info: (String, OsString) = (String::from(""), OsString::from(""));

        if valid_external_commands.contains_key(command) {
            let command_borrowed = valid_external_commands.get_key_value(command).unwrap();
            command_info.0 = command_borrowed.0.to_string();
            command_info.1 = command_borrowed.1.to_owned();
        }

        match command {
            cmd if !COMMANDS.contains(&cmd) && !valid_external_commands.contains_key(cmd) => {
                Ok(Box::new(InvalidCommand {
                    args: cmd.to_string(),
                }))
            }
            "exit" => Ok(Box::new(ExitCommand { args: args.clone() })),
            "echo" => Ok(Box::new(EchoCommand { args: args.clone() })),
            "type" => Ok(Box::new(TypeCommand {
                args: args.clone(),
                valid_commands: HashMap::with_capacity(100),
            })),
            cmd if valid_external_commands.contains_key(cmd) => Ok(Box::new(RunCommand {
                args: args.clone(),
                command: command_info,
            })),

            _ => todo!(),
        }
    }
}

impl Parse for RunCommand {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        Ok(Box::new(Self {
            args: self.args.clone(),
            command: self.command.clone(),
        }))
    }
}

impl Parse for InvalidCommand {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        Ok(Box::new(Self {
            args: self.args.clone(),
        }))
    }
}

impl Parse for ExitCommand {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        Ok(Box::new(Self {
            args: self.args.clone(),
        }))
    }
}

impl Parse for EchoCommand {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        Ok(Box::new(Self {
            args: self.args.clone(),
        }))
    }
}

impl Parse for TypeCommand {
    fn parse(&self) -> Result<Box<dyn Execute>, Box<dyn error::Error>> {
        let pbs = get_executables_from_paths(get_paths())?;

        Ok(Box::new(TypeCommand {
            args: self.args.clone(),
            valid_commands: pbs,
        }))
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
        let output = Command::new(self.command.1.clone())
            .args(self.args.clone())
            .output()
            .expect("Failed");

        io::stdout().write_all(&output.stdout).unwrap();
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
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let valid_commands = get_executables_from_paths(get_paths()).unwrap_or(HashMap::new());
        Shell::initiate(input.trim().to_string(), valid_commands)?
            .parse()?
            .execute();
    }
}
