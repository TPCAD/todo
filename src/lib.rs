use crate::item::Todo;
use chrono::Local;
use config::Config;
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    process::exit,
};

pub mod config;
pub mod item;
pub mod misc;

pub fn run(action: &str, target: &str, config: &Config) {
    match action {
        "" | "help" => {
            misc::help_message();
        }
        "add" | "a" => {
            add_todo(target, config);
        }
        "done" | "d" => remove_todo(config, target),
        "list" | "ls" => list_todos(config),
        _ => {}
    }
}

fn add_todo(content: &str, config: &Config) {
    if content.is_empty() {
        println!("Content can't be empty!");
        exit(1);
    }

    let date = Local::now();
    let date = date.format("%Y-%m-%d %H:%M:%S").to_string();

    let item = Todo::new(content, date);
    let item = item.to_record();

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(config.todo_record_path())
    {
        writeln!(file, "{}", item).expect("Failed to write into todos.txt");
        println!("Success!");
    } else {
        println!("Failed to open todos.txt");
    };
}

fn remove_todo(config: &Config, target: &str) {
    let index: usize;

    if let Ok(inner) = target.parse::<usize>() {
        index = inner;
    } else {
        if !target.is_empty() {
            println!("Index must be an integer");
            exit(1);
        }
        index = 0;
    }

    let todos = read_todo_record(config);

    if index >= todos.len() {
        println!("Out of index");
        exit(1);
    }

    // clear todos.txt
    fs::write(config.todo_record_path(), "").expect("Failed to write in todos.txt");

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .open(config.todo_record_path())
    {
        for (i, k) in todos.iter().enumerate() {
            if i == index {
                continue;
            }
            writeln!(file, "{k}").unwrap();
        }
        println!("Success!");
    }
}

fn list_todos(config: &Config) {
    let todos = read_todo_record(config);

    for (i, k) in todos.iter().enumerate() {
        println!("{i}. {k}");
    }
}

fn read_todo_record(config: &Config) -> Vec<String> {
    if !config.todo_record_path().try_exists().unwrap() {
        println!("No todos.txt. Please add one todo first.");
        exit(1);
    }
    read_to_string(config.todo_record_path())
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
