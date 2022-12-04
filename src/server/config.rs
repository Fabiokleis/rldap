use std::path::{Path, PathBuf};
use std::env;
use std::collections::hash_map;
use std::fs;

/// Config constructor to load environment variables
#[derive(Default, Clone)]
pub struct Config {
    file_path: Option<String>,
    vars: hash_map::HashMap<String, String>,
}

impl Config {
    /// Creates Config Struct by given path
    pub fn from_path(file_path: Option<String>) -> Self {
        Config { file_path, vars: hash_map::HashMap::new()}
    }

    /// Load environment variables and populate vars HashMap
    pub fn init(&mut self) -> Result<(), std::io::Error> {
        let path: String = match self.file_path.clone() {
            Some(v) => v,
            None => String::from(""),
        };
        let path = Path::new(&path).join("./.env");

        load_from_path(Path::new(&path).to_path_buf())?;
        for (key, value) in env::vars() {
            self.vars.insert(key, value);
        }
        Ok(())
    }

    /// Returns a String value from a key 
    pub fn get_var(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }
}

/// Open a given path file and exports on env::vars
pub fn load_from_path(path: PathBuf) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let key_values: Vec<(&str, &str)> = contents.lines().map(|l| l.split_once('=').unwrap()).collect();
    key_values.iter().for_each(|(k, v)| env::set_var(k, v));
    Ok(())
}
