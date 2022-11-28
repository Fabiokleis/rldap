use std::path::Path;
use std::env;
use std::collections::hash_map;

#[derive(Default, Clone)]
pub struct Config {
    file_path: String,
    vars: hash_map::HashMap<String, String>,
}

impl Config {
    pub fn from_path(file_path: String) -> Self {
        Config { file_path, vars: hash_map::HashMap::new() }
    }

    pub fn init(&mut self) {
        let path = Path::new(&self.file_path).join("./.env");
        dotenv::from_path(Path::new(&path)).ok();
        for (key, value) in env::vars() {
            self.vars.insert(key, value);
        }
    }

    pub fn get_var(&self, key: &str) -> Option<String> {
        self.vars.get(key).map(|value| value.clone())
    }
}
