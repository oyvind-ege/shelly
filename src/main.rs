#[allow(unused_imports)]
use std::io::{self, Write};

struct Command<'a> {
    raw_cmd: String,
    cmd: &'a str,
    args: &'a str, //probably needs a generic later on
}

#[derive(Debug)]
enum ProgramError {
    IOError(std::io::Error),
}

// impl<'a> Command<'a> {
//     fn parse(input: String) -> Self {
//         // From "exit 0" to "Command { cmd: "exit", args: vec![0]}"
//         let split_string: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();
//         let command = Command {
//             raw_cmd: input.clone(),
//             cmd: split_string[0],
//             args: split_string[1],
//         };
//         command
//     }
// }

fn main() -> Result<(), ProgramError> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();

        if command[0] == "exit" && command[1] == "0" {
            return Ok(());
        }
        println!("{}: command not found", input.trim());
    }
}
