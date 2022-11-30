use std::path::Path;
use std::env;
use std::collections::hash_map;

#[derive(Default, Clone)]
pub struct Config {
    file_path: Option<String>,
    vars: hash_map::HashMap<String, String>,
    from_path: bool,
}

impl Config {
    pub fn from_path(file_path: Option<String>, from_path: bool) -> Self {
        Config { file_path, vars: hash_map::HashMap::new(), from_path}
    }

    pub fn init(&mut self) {
        if self.from_path {
            let path: String = match self.file_path.clone() {
                Some(v) => v,
                None => String::from(""),
            };
            let path = Path::new(&path).join("./.env");
            dotenv::from_path(Path::new(&path)).ok();
        } else {
            dotenv::dotenv().ok();
        }
        for (key, value) in env::vars() {
            self.vars.insert(key, value);
        }
    }

    pub fn get_var(&self, key: &str) -> Option<String> {
        self.vars.get(key).map(|value| value.clone())
    }
}
