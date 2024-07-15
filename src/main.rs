use std::env;
use todo::commands::{add, done, list, remove, reset, restore, sort};
use todo::config::Config;
use todo::help;
use todo::{prepare_config, prepare_todo};

fn main() {
    // ensure config directory is exist
    prepare_config();
    let config = Config::from_file();
    prepare_todo(config.todo_path());

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => list(&config),
            "add" => add(&config, &args[2..]),
            "rm" => remove(&config, &args[2..]),
            "done" => done(&config, &args[2..]),
            // "raw" => todo.raw(&args[2..]),
            "sort" => {
                sort(&config);
                list(&config);
            }
            "reset" => reset(&config),
            "restore" => restore(&config),
            _ => help(),
        }
    } else {
        list(&config);
    }
}
