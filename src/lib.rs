use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn get_executables_from_paths(pbs: Vec<PathBuf>) -> io::Result<HashMap<String, OsString>> {
    let mut executables: HashMap<String, OsString> = HashMap::new();
    for dir in pbs {
        if dir.is_dir() {
            let directory_path = dir.clone().into_os_string();
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
                    executables.insert(binary_name, directory_path.clone());
                }
            }
        }
    }
    Ok(executables)
}

#[cfg(test)]
mod test_get_executables {
    use crate::get_executables_from_paths;
    use crate::HashMap;
    use crate::OsString;
    use crate::PathBuf;

    #[test]
    fn test() {
        let path = OsString::from("/Users/elgen/testdir");
        let filename = String::from("hello.bin");
        let pb = vec![PathBuf::from(path.clone())];
        assert_eq!(
            get_executables_from_paths(pb).unwrap(),
            HashMap::from([(filename, path)])
        )
    }

    #[test]
    fn no_dir() {
        let path = OsString::from("/Users/elgen/none/invalid");
        let filename = String::from("testing.bin");
        let pb = vec![PathBuf::from(path.clone())];
        assert_eq!(get_executables_from_paths(pb).unwrap(), HashMap::new());
    }
}
