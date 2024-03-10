use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config<'a> {
    config_dir: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(config_dir: &'a str) -> Config<'a> {
        Config {
            config_dir: Path::new(config_dir),
        }
    }

    pub fn create_config_file(&self) {
        if let Err(e) = fs::create_dir(self.config_dir) {
            match e.kind() {
                ErrorKind::AlreadyExists => {}
                _ => println!("Failed to create config directory ~/.config/todo"),
            }
        };
    }

    pub fn todo_record_path(&self) -> PathBuf {
        Path::new(self.config_dir).join("todos.txt")
    }

    // pub fn done_record_path(&self) -> PathBuf {
    //     Path::new(self.config_dir).join("dones.txt")
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_todo_record_path() {
        let config = Config::new("/home/tpcad/.config/todo");
        assert_eq!(
            PathBuf::from("/home/tpcad/.config/todo/todos.txt"),
            config.todo_record_path()
        );
    }
}
