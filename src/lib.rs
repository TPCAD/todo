use colored::*;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
    process,
};

#[derive(Debug)]
pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();

                let legacy_todo = format!("{}/TODO", &home);
                // TODO: try_exists
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.todo", &home),
                }
            }
        };

        let todo_bak = match env::var("TODO_BAK_DIR") {
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak"),
        };

        let no_backup = env::var("TODO_NOBACKUP").is_ok();

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            // TODO: truncate
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the todofile");

        // Create a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo = contents.lines().map(str::to_string).collect();

        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

    pub fn list(&self) {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for (number, task) in self.todo.iter().enumerate() {
            if task.len() > 4 {
                let number = (number + 1).to_string().bold();
                let symbol = &task[..4];
                let task = &task[4..];

                if symbol == "[*] " {
                    data = format!("{} {}\n", number, task.strikethrough());
                } else if symbol == "[ ] " {
                    data = format!("{} {}\n", number, task);
                }

                writer
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(&todofile);

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // TODO: not clear
    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}\n", arg.len());
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            let stdout = io::stdout();
            let mut writer = BufWriter::new(stdout);

            for task in self.todo.iter() {
                let mut data = String::new();
                if task.len() > 5 {
                    let symbol = &task[..4];
                    let task = &task[4..];

                    if symbol == "[*] " && arg[0] == "done" {
                        data = format!("{task}\n");
                    } else if symbol == "[ ] " && arg[0] == "todo" {
                        data = format!("{task}\n");
                    }
                }
                writer
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
    }

    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(&todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    fn remove_file(&self) {
        match fs::remove_file(&self.todo_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while clearing todo file: {e}");
            }
        };
    }

    pub fn reset(&self) {
        if !self.no_backup {
            match fs::copy(&self.todo_path, &self.todo_bak) {
                Ok(_) => self.remove_file(),
                Err(_) => {
                    eprintln!("Couldn't backup the todo file");
                }
            }
        } else {
            self.remove_file();
        }
    }

    pub fn restore(&self) {
        fs::copy(&self.todo_bak, &self.todo_path).expect("unable to restore the todo file");
    }

    pub fn sort(&self) {
        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line);
                } else if &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    done.push_str(&line);
                }
            }
        }

        let newtodo = format!("{}{}", &todo, &done);

        let mut todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        todofile
            .write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .write(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                if args.contains(&(pos + 1).to_string()) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    }
                } else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("unable to write data");
                }
            }
        }
    }
}

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
