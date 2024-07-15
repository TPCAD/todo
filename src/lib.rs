pub mod commands;
pub mod config;
pub mod item;

use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

// TODO: not clear
// pub fn raw(&self, arg: &[String]) {
//     if arg.len() > 1 {
//         eprintln!("todo raw takes only 1 argument, not {}\n", arg.len());
//     } else if arg.is_empty() {
//         eprintln!("todo raw takes 1 argument (done/todo)");
//     } else {
//         let stdout = io::stdout();
//         let mut writer = BufWriter::new(stdout);
//
//         for task in self.todo.iter() {
//             let mut data = String::new();
//             if task.len() > 5 {
//                 let symbol = &task[..4];
//                 let task = &task[4..];
//
//                 if symbol == "[*] " && arg[0] == "done" {
//                     data = format!("{task}\n");
//                 } else if symbol == "[ ] " && arg[0] == "todo" {
//                     data = format!("{task}\n");
//                 }
//             }
//             writer
//                 .write_all(data.as_bytes())
//                 .expect("Failed to write to stdout");
//         }
//     }
// }

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore 
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";
pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}

pub fn prepare_config() {
    // get path
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("todo");

    // create directories if not exist
    fs::create_dir_all(&path).expect("Failed to create directories");

    // create config.toml if not exist and write default config
    path.push("config.toml");
    if !path.exists() {
        let config_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect("Faild to create or open config file");

        let mut buffer = BufWriter::new(config_file);

        buffer
            .write_all(DEFAULT_CONFIG.as_bytes())
            .expect("Failed to write config file");
    }
}

pub fn prepare_todo(path: &str) {
    // create todo.toml if not exist
    let path = PathBuf::from(path);
    if !path.exists() {
        let _ = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect("Faild to create or open todo file");
    }
}

pub fn write_file(buf: &str, path: &str) {
    let todofile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Couldn't open the todofile");

    let mut buffer = BufWriter::new(&todofile);

    buffer
        .write_all(buf.as_bytes())
        .expect("unable to write data");
}

const DEFAULT_CONFIG: &str = "[general]
todo_path = \"~/.config/todo/todo.toml\"
bak_path = \"~/.cache/todo.bak\"
editor = \"nvim\"
backup = true

[command.list]
list_date = true
list_priority = false

[command.add]
continuous_contents = true

[command.done]
remove_while_done = false";
