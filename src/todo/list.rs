#![allow(dead_code, unused)]

use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use super::Item;

pub struct List {
    list: Vec<Item>,
}

impl List {
    pub fn new() -> Self {
        List { list: Vec::new() }
    }

    pub fn from_file(save_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        // read save dir
        let mut file = File::open(save_path)?;
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;

        let mut ivec: Vec<Item> = Vec::new();

        // parse read content
        for line in buf.lines() {
            if line == "" {
                continue;
            }

            let vline = line.split("|").collect::<Vec<&str>>();

            let title = match vline.get(0) {
                Some(v) => v.trim().to_string(),
                None => return Err(Box::from("failed to parse title saved file")),
            };

            let status_done = match vline.get(1) {
                Some(v) => {
                    let v = v.trim().to_string();

                    match v.as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return Err(Box::from("failed to parse status saved file")),
                    }
                }
                None => return Err(Box::from("failed to parse saved file")),
            };

            ivec.push(Item { title, status_done })
        }

        Ok(List { list: ivec })
    }

    pub fn add(&mut self, todo_title: &str) {
        let item = Item::new(todo_title);
        self.list.push(item);
    }

    pub fn delete(&mut self, todo_id: usize) {
        self.list.remove(todo_id - 1);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn update_item_title(&mut self, todo_id: usize, title: &str) -> Result<(), Box<dyn Error>> {
        let idx = todo_id - 1;

        let mut item = match self.list.get(idx) {
            Some(item) => item.clone(),
            None => {
                return Err(Box::from("failed to get item"));
            }
        };

        item.update_title(title);

        self.delete(todo_id);
        self.list.insert(idx, item);

        Ok(())
    }

    pub fn update_item_status(
        &mut self,
        todo_id: usize,
        status: bool,
    ) -> Result<(), Box<dyn Error>> {
        let idx = todo_id - 1;

        let mut item = match self.list.get(idx) {
            Some(item) => item.clone(),
            None => {
                return Err(Box::from("failed to get item"));
            }
        };

        item.update_status(status);

        self.delete(todo_id);
        self.list.insert(idx, item);

        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();

        for item in self.list.iter() {
            str.push_str(item.to_string().as_str());
            str.push_str("\n");
        }

        str
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, item) in self.list.iter().enumerate() {
            let status_str = if item.status_done {
                "done"
            } else {
                "in progress"
            };
            write!(f, "{}: {} | {}\n", idx + 1, item.title, status_str)?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List::new();

        list.add("study");
        list.add("exercise");

        assert_eq!(2, list.len());

        println!("{list}");

        list.update_item_title(1, "sauna");

        println!("updated item title here\n{list}");

        list.delete(1);
        assert_eq!(1, list.len());

        println!("deleted item here\n{list}");
        //
        // list.update_item_status(1);
        // println!("updated item here\n{list}");
        //
        // list.update_item_status(1);
        // println!("updated item here\n{list}");

        list.add("study database");
        list.add("watch youtube");

        println!("-------------------------------------------------------");
        println!("{}", list.to_string());
    }

    #[test]
    fn test_from() {
        let spath = crate::get_save_file().unwrap();
        let list = List::from_file(&spath).unwrap();

        for (i, item) in list.list.iter().enumerate() {
            println!("item {}", i);
            println!("title: {}", item.title);
            println!("status: {}\n", item.status_done);
        }
    }
}
