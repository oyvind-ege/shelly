use codecrafters_shell::*;
use std::io::{self, Write};
extern crate exitcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        Shell::initiate(input.trim().to_string())?.execute();
    }
}
