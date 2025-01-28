use std::io::{self, Write};
use std::process::{self};

extern crate exitcode;

trait Execute {
    fn execute(&self);
}

#[derive(Debug)]
enum CommandKind {
    Type(TypeCommand),
    Exit(ExitCommand),
    Echo(EchoCommand),
    Invalid(String),
}

#[derive(Debug)]
struct Command<'a> {
    cmd: String,
    args: Vec<&'a str>,
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
struct TypeCommand {}

#[derive(Debug)]
struct InvalidCommand {
    args: Vec<String>,
}

impl<'a> Command<'a> {
    fn new() -> Self {
        Command {
            cmd: "Invalid".to_string(),
            args: vec![""],
        }
    }

    fn parse(&mut self, input: String) -> Box<dyn Execute> {
        let split_input = input.split_whitespace().collect::<Vec<&str>>();

        self.args = if split_input.len() > 1 {
            split_input[1..].to_vec()
        } else {
            [""].to_vec()
        };

        match split_input[0] {
            "exit" => Box::new(ExitCommand {
                args: self.args.clone(),
            }),
            "echo" => Box::new(EchoCommand {
                args: self.args.clone(),
            }),
            cmd => Box::new(InvalidCommand {
                args: [cmd.to_string()].to_vec(),
            }),
        }
    }
}

impl Execute for ExitCommand {
    fn execute(&self) {
        match self.args.first().unwrap() {
            val if val == "0" => process::exit(exitcode::OK),
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
        println!("{:?}: command not found", self.args);
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

// command.parse(input.trim()).execute();

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {}
}
