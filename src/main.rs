use std::env;

use todo::{config::Config, misc, run};

fn main() {
    let (action, target) = misc::parse_args();

    let home_dir = env::var("HOME").unwrap();
    let home_dir = home_dir + "/.config/todo";
    let config = Config::new(&home_dir);
    config.create_config_file();

    run(&action, &target, &config);
}
