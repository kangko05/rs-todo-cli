use std::{fs, path::PathBuf};

pub mod todo;

pub struct DirManager {
    save_path: String,
}

impl DirManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let save_path = String::from("todo_records");
        let save_path = PathBuf::from(save_path);

        let rd = fs::read_dir(&save_path)?
            .map(|entry| entry.expect(""))
            .collect::<Vec<fs::DirEntry>>();

        for entry in rd {
            let fpath = PathBuf::from(entry.file_name());
            let content =  fs::read_to_string(fpath)
        }

        Ok(DirManager {
            save_path: String::from(""),
        })
    }
}

#[cfg(test)]
mod dir_tests {
    use super::*;

    #[test]
    fn test_dir() {
        let _ = DirManager::new();
    }
}
