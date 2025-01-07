#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        println!("{}: command not found", input.trim());
    }
}

fn validate_command(cmd: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::validate_command;

    #[test]
    fn test_basic() {
        assert!(!validate_command("test"));
        assert!(!validate_command("invalid command"));
    }
}
