use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

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
