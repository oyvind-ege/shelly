use std::io::{self, Write};
use std::process::{self};

static COMMANDS: [&str; 3] = ["exit", "echo", "type"];

extern crate exitcode;

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
    args: String,
}

#[derive(Debug)]
struct InvalidCommand {
    args: String,
}

impl Command {
    fn new() -> Self {
        Command { args: vec![] }
    }

    fn parse(&mut self, input: String) -> Box<dyn Execute> {
        let split_input = input.split_whitespace().collect::<Vec<&str>>();

        self.args = if split_input.len() > 1 {
            split_input[1..]
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
        } else {
            [].to_vec()
        };

        match split_input[0] {
            cmd if !COMMANDS.contains(&cmd) => Box::new(InvalidCommand {
                args: cmd.to_string(),
            }),
            "exit" => Box::new(ExitCommand {
                args: self.args.clone(),
            }),
            "echo" => Box::new(EchoCommand {
                args: self.args.clone(),
            }),
            "type" => Box::new(TypeCommand {
                args: self.args.first().unwrap().to_string(),
            }),
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

impl Execute for TypeCommand {
    fn execute(&self) {
        match &self.args {
            arg if !COMMANDS.contains(&arg.as_str()) => println!("{}: not found", arg),
            arg => println!("{} is a builtin", arg),
        };
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        Command::new().parse(input.trim().to_string()).execute();
    }
}
