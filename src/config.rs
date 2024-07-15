use serde::Deserialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    general: GeneralConfig,
    command: CommandConfig,
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    todo_path: String,
    bak_path: String,
    editor: String,
    backup: bool,
}

#[derive(Debug, Deserialize)]
struct CommandConfig {
    list: ListConfig,
    add: AddConfig,
    done: DoneConfig,
}

#[derive(Debug, Deserialize)]
struct ListConfig {
    list_date: bool,
    list_priority: bool,
}

#[derive(Debug, Deserialize)]
struct AddConfig {
    continuous_contents: bool,
}

#[derive(Debug, Deserialize)]
struct DoneConfig {
    remove_while_done: bool,
}

impl Config {
    pub fn from_file() -> Config {
        // get path
        let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".config");
        path.push("todo");
        path.push("config.toml");

        // TODO: util function `read_file`
        // read confit.toml
        let mut buf = String::new();
        let _ = OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("Failed to open config file")
            .read_to_string(&mut buf);

        // parse config
        let mut config = toml::from_str::<Config>(&buf).expect("Faild to parse config file");

        let home_dir = env::var("HOME").unwrap();
        config.set_todo_path(&config.todo_path().replace('~', &home_dir));
        config.set_bak_path(&config.bak_path().replace('~', &home_dir));

        config
    }

    pub fn todo_path(&self) -> &str {
        &self.general.todo_path
    }
    pub fn set_todo_path(&mut self, path: &str) {
        self.general.todo_path = path.to_string()
    }
    pub fn bak_path(&self) -> &str {
        &self.general.bak_path
    }
    pub fn set_bak_path(&mut self, path: &str) {
        self.general.bak_path = path.to_string()
    }
    pub fn editor(&self) -> &str {
        &self.general.editor
    }
    pub fn backup(&self) -> bool {
        self.general.backup
    }
    pub fn list_date(&self) -> bool {
        self.command.list.list_date
    }
    pub fn list_priority(&self) -> bool {
        self.command.list.list_priority
    }
    pub fn continuous_contents(&self) -> bool {
        self.command.add.continuous_contents
    }
    pub fn remove_while_done(&self) -> bool {
        self.command.done.remove_while_done
    }
}
