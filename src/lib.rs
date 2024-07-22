use chrono::{offset::Local, Datelike};
use std::{
    env,
    error::Error,
    fs::{create_dir_all, File},
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

pub mod todo;

pub fn get_date_str() -> String {
    // get current date to use it as file name
    let year = Local::now().year();
    let month = Local::now().month();
    let day = Local::now().day();

    format!("{year}_{month}_{day}")
}

pub fn get_save_file() -> Result<PathBuf, Box<dyn Error>> {
    // let mut curr = env::current_dir()?;
    // curr.push("todo_save");

    // temporarily save dir is fixed to $HOME/todo_save
    let home = env::var("HOME").expect("failed to get $HOME");
    let mut curr = PathBuf::from(home);
    curr.push("todo_save");

    // check if save dir exists
    // if it doesn't, create one
    if !curr.exists() {
        create_dir_all(&curr)?;
    }

    let date_str = get_date_str();
    curr.push(date_str);

    Ok(curr)
}

pub fn save_to_file(fpath: &PathBuf, content: &str) -> Result<(), Box<dyn Error>> {
    // if dir does not exist, create one
    let file = File::create(fpath)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", content)?;

    Ok(())
}

pub fn read_file_contents(fpath: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(fpath)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf).expect("failed to read file");

    Ok(buf)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_file() {
        let curr = get_save_file().unwrap();
        println!("{}", curr.to_string_lossy());

        //save_to_file(&curr, "hello world!\ntesting...").unwrap();
        let r = read_file_contents(&curr).unwrap();
        println!("read: {r}");
    }
}
