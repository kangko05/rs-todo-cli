#![allow(dead_code, unused)]

use std::fmt::Display;

pub struct Item {
    pub title: String,
    pub status_done: bool,
}

impl Item {
    pub fn new(title: &str) -> Item {
        Item {
            title: String::from(title),
            status_done: false,
        }
    }

    pub fn update_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn update_status(&mut self, status: bool) {
        self.status_done = status;
    }

    pub fn to_string(&self) -> String {
        format!("{} | {}", self.title, self.status_done)
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            title: self.title.clone(),
            status_done: self.status_done,
        }
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn test_item() {
        let it = Item::new("walk the dog");

        assert_eq!(String::from("walk the dog"), it.title);
        assert!(!it.status_done);
    }
}
