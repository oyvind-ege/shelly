use std::io::{self, Write};
use std::process::{self};

extern crate exitcode;

#[derive(Debug)]
enum CommandKind {
    Exit,
    Echo,
    Invalid(String),
}

#[derive(Debug)]
struct Command<'a> {
    cmd: CommandKind,
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn new() -> Self {
        Command {
            cmd: CommandKind::Invalid(String::from("none")),
            args: vec![""],
        }
    }

    fn parse(&mut self, input: &'a str) -> &mut Self {
        let split_input = input.split_whitespace().collect::<Vec<&'a str>>();
        match split_input[0] {
            "exit" => self.cmd = CommandKind::Exit,
            "echo" => self.cmd = CommandKind::Echo,
            cmd => self.cmd = CommandKind::Invalid(String::from(cmd)),
        };

        if split_input.len() > 1 {
            self.args = split_input[1..].to_vec();
        };
        self
    }

    fn execute(&self) {
        match &self.cmd {
            CommandKind::Exit if self.args.first().unwrap() == &"0" => process::exit(exitcode::OK),
            CommandKind::Echo => {
                println!("{}", self.args.join(" "));
            }
            CommandKind::Exit => process::exit(exitcode::SOFTWARE),
            CommandKind::Invalid(cmd) => println!("{}: command not found", cmd),
        }
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
        let mut command: Command = Command::new();
        command.parse(input.trim()).execute();
    }
}

// command.parse(input.trim()).execute();

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {}
}
