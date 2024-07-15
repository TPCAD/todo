use std::{fs::OpenOptions, io::Read};

use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Debug, Deserialize, Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    date: Datetime,
    contents: String,
    priority: String,
    done: bool,
}

impl Items {
    pub fn from_file(path: &str) -> Vec<Item> {
        let mut buf = String::new();
        let _ = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open todo file")
            .read_to_string(&mut buf);

        match toml::from_str::<Items>(&buf) {
            Ok(items) => items.items,
            // Err(_) => Vec::<Item>::new(),
            Err(_) => Vec::<Item>::new(),
        }
    }

    pub fn new(items: Vec<Item>) -> Items {
        Items { items }
    }
}

impl Item {
    pub fn date(&self) -> String {
        format!("{}", self.date)
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn priority(&self) -> &str {
        &self.priority
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, value: bool) {
        self.done = value
    }

    pub fn new(date: Datetime, contents: String, priority: String, done: bool) -> Item {
        Item {
            date,
            contents,
            priority,
            done,
        }
    }
}
