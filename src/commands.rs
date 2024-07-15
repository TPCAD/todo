use crate::{
    config::Config,
    item::{Item, Items},
    write_file,
};
use chrono::{Datelike, Local};
use colored::Colorize;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    process,
};
use toml::value::Date;

pub fn list(config: &Config) {
    let items = Items::from_file(config.todo_path());

    if items.is_empty() {
        println!("Clear!");
        return;
    }

    for (idx, item) in items.iter().enumerate() {
        let mut buf = format!("{}.", idx.to_string().bold());

        if config.list_date() {
            buf = format!("{buf} {:10}", item.date())
        }

        if config.list_priority() {
            let priority = match item.priority() {
                "MEDIUM" => "MEDIUM".to_string().yellow().bold(),
                "HIGH" => "HIGH".to_string().red().bold(),
                _ => "LOW".to_string().green().bold(),
            };

            buf = format!("{buf} {:6}", priority);
        }

        buf = format!("{buf} {}", item.contents());
        if item.done() {
            buf = buf.strikethrough().to_string();
        }
        println!("{buf}")
    }
}

pub fn add(config: &Config, args: &[String]) {
    if args.is_empty() {
        eprintln!("todo add takes at least 1 argument");
        process::exit(1);
    }

    let priority = match args[0].as_str() {
        "MEDIUM" => "MEDIUM".to_string(),
        "HIGH" => "HIGH".to_string(),
        _ => "LOW".to_string(),
    };

    let date = Local::now();

    // TODO: so ugly code
    let args = match args[0].as_str() {
        "LOW" | "MEDIUM" | "HIGH" => &args[1..],
        _ => args,
    };

    if config.continuous_contents() {
        let mut contents = String::new();
        for arg in args {
            contents = format!("{contents} {arg}")
        }
        contents = contents.trim().to_string();

        let items = Items::new(vec![Item::new(
            toml::value::Datetime {
                date: Some(Date {
                    year: date.year() as u16,
                    month: date.month() as u8,
                    day: date.day() as u8,
                }),
                time: None,
                offset: None,
            },
            contents,
            priority,
            false,
        )]);
        let toml_str = toml::to_string_pretty(&items).expect("Failed to parse toml to string");

        write_file(&toml_str, config.todo_path());
    } else {
        for arg in args {
            let contents = arg.clone();
            let items = Items::new(vec![Item::new(
                toml::value::Datetime {
                    date: Some(Date {
                        year: date.year() as u16,
                        month: date.month() as u8,
                        day: date.day() as u8,
                    }),
                    time: None,
                    offset: None,
                },
                contents,
                priority.clone(),
                false,
            )]);

            let toml_str = toml::to_string_pretty(&items).expect("Failed to parse toml to string");

            write_file(&toml_str, config.todo_path());
        }
    }
}

pub fn done(config: &Config, args: &[String]) {
    if args.is_empty() {
        eprintln!("todo done takes at least 1 argument");
        process::exit(1);
    }
    let mut items = Items::from_file(config.todo_path());

    if config.remove_while_done() {
        items = items
            .into_iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if args.contains(&idx.to_string()) {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();
    } else {
        for (idx, item) in items.iter_mut().enumerate() {
            if args.contains(&idx.to_string()) {
                item.set_done(true)
            }
        }
    }

    let items = Items::new(items);

    let toml_str = toml::to_string_pretty(&items).expect("Failed to parse toml to string");

    let todofile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config.todo_path())
        .expect("Couldn't open the todofile");

    let mut buffer = BufWriter::new(&todofile);

    buffer
        .write_all(toml_str.as_bytes())
        .expect("unable to write data");
}

pub fn remove(config: &Config, args: &[String]) {
    if args.is_empty() {
        eprintln!("todo done takes at least 1 argument");
        process::exit(1);
    }
    let mut items = Items::from_file(config.todo_path());

    items = items
        .into_iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if args.contains(&idx.to_string()) {
                None
            } else {
                Some(item)
            }
        })
        .collect();

    let items = Items::new(items);

    let toml_str = toml::to_string_pretty(&items).expect("Failed to parse toml to string");

    let todofile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config.todo_path())
        .expect("Couldn't open the todofile");

    let mut buffer = BufWriter::new(&todofile);

    buffer
        .write_all(toml_str.as_bytes())
        .expect("unable to write data");
}

pub fn reset(config: &Config) {
    if config.backup() {
        match fs::copy(config.todo_path(), config.bak_path()) {
            Ok(_) => remove_file(config.todo_path()),
            Err(_) => {
                eprintln!("Couldn't backup the todo file");
            }
        }
    }
}

fn remove_file(path: &str) {
    match fs::remove_file(path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error while clearing todo file: {e}");
        }
    };
}

pub fn restore(config: &Config) {
    fs::copy(config.bak_path(), config.todo_path()).expect("unable to restore the todo file");
}

pub fn sort(config: &Config) {
    let items = Items::from_file(config.todo_path());

    let (todo, done): (Vec<_>, Vec<_>) = items.into_iter().partition(|item| !item.done());

    let items = todo.into_iter().chain(done).collect();
    let items = Items::new(items);

    let toml_str = toml::to_string_pretty(&items).expect("Failed to parse toml to string");

    let todofile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config.todo_path())
        .expect("Couldn't open the todofile");

    let mut buffer = BufWriter::new(&todofile);

    buffer
        .write_all(toml_str.as_bytes())
        .expect("unable to write data");
}

// pub fn raw(config: &Config) {}
