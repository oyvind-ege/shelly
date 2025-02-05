use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CommandInfo {
    pub bin: String,
    pub path: OsString,
}

pub fn get_executables_from_paths(pbs: Vec<PathBuf>) -> io::Result<HashMap<String, OsString>> {
    let mut executables: HashMap<String, OsString> = HashMap::new();
    for dir in pbs {
        if dir.is_dir() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let binary_name = path
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap();

                    executables
                        .entry(binary_name)
                        .or_insert(path.clone().into_os_string());
                }
            }
        }
    }
    Ok(executables)
}

pub fn get_paths() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(v) => env::split_paths(&v).collect(),

        None => todo!(),
    }
}

pub fn parse_command_and_arguments(input: &str) -> (&str, Vec<String>) {
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

    (command, args)
}

pub fn get_command_info(
    valid_external_commands: &HashMap<String, OsString>,
    command: &str,
) -> CommandInfo {
    let command_borrowed = valid_external_commands.get_key_value(command).unwrap();

    CommandInfo {
        bin: command_borrowed.0.to_string(),
        path: command_borrowed.1.to_owned(),
    }
}

#[cfg(test)]
mod tests {

    use super::parse_command_and_arguments;

    #[test]
    fn test_parse_command() {
        let input = String::from("cmd x y z");
        let cmd = "cmd";
        let args = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn test_empty_args() {
        let input = String::from("cmd");
        let cmd = "cmd";
        let args: Vec<String> = vec![];
        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }

    #[test]
    fn test_whitespace() {
        let input = String::from("cmd   ");
        let cmd = "cmd";
        let args: Vec<String> = vec![];

        assert_eq!(parse_command_and_arguments(&input), (cmd, args));
    }
}
