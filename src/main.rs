use codecrafters_shell::*;
use std::io::{self, Write};
extern crate exitcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shell = Shell::init();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        shell.parse(input.trim().to_string())?.execute()?;
    }
}
