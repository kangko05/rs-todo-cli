#![allow(dead_code, unused)]

use std::{error::Error, fmt::Display, fs};

use super::Item;

pub struct List {
    list: Vec<Item>,
}

impl List {
    pub fn new() -> List {
        List { list: Vec::new() }
    }

    pub fn from() {
        // read save dir
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

    pub fn update_item_title(&mut self, todo_id: usize, title: &str) {
        let idx = todo_id - 1;
        let new_item = Item::new(title);

        self.delete(todo_id);
        self.list.insert(idx, new_item);
    }

    pub fn update_item_status(&mut self, todo_id: usize) -> Result<(), Box<dyn Error>> {
        let idx = todo_id - 1;

        let mut item = match self.list.get(idx) {
            Some(item) => item.clone(),
            None => {
                return Err(Box::from("failed to get item"));
            }
        };

        item.update_status();

        self.delete(todo_id);
        self.list.insert(idx, item);

        Ok(())
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

        list.update_item_status(1);
        println!("updated item here\n{list}");

        list.update_item_status(1);
        println!("updated item here\n{list}");
    }
}
