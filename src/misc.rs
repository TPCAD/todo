use std::{collections::VecDeque, env};

// parse arguments
pub fn parse_args() -> (String, String) {
    let mut args: VecDeque<String> = env::args().collect();

    args.pop_front();

    (
        // get action
        args.pop_front().unwrap_or("".to_owned()),
        // get target
        args.into_iter().collect::<Vec<String>>().join(" "),
    )
}

// help message
pub fn help_message() {
    println!(
        r#"Usage: todo <action> [<content>|<index>]
Record something to do.

Action:
    add                     add a todo
    done                    remove a todo
    list                    list all todos

Target:
    content                 use with add action, content to do
    index                   use with done action, start from 0

Examples:
    todo add go shopping    add \"go shopping\"
    todo done 2             remove todo which index is 2
    todo list               list all todos"#
    );
}
