use std::{env, fmt::Display};
mod config;
use config::Config;

#[derive(Default, Clone)]
pub struct Server {
    config: Config,
    ldap_server: String,
    base_dn: String,
    bind_dn: String,
    auth_pass: String,
    filter: String,
    attributes: Vec<&'static str>,
}

pub fn configure_env(server: &mut Server, from_path: bool) {
    let config = Config::from_path(env::var("CARGO_MANIFEST_DIR").ok(), from_path);
    server.set_config(config);
    server.load_env_variables();
}

impl Server {

    pub fn ldap_server(&self) -> String {
        self.ldap_server.clone()
    }

    pub fn base_dn(&self) -> String {
        self.base_dn.clone()
    }

    pub fn filter(&self) -> String {
        self.filter.clone()
    }

    pub fn attribs(&self) -> Vec<&'static str> {
        self.attributes.clone()
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn load_env_variables(&mut self) {
        self.config.init();
        self.ldap_server = self.config.get_var("ldap_server").unwrap();
        self.base_dn = self.config.get_var("base_dn").unwrap();
        self.bind_dn = self.config.get_var("bind_dn").unwrap();
        self.auth_pass = self.config.get_var("auth_pass").unwrap();
    }

    pub fn set_filter(&mut self, filter: &str) {
        self.filter = String::from(filter);
    }

    pub fn set_attribs(&mut self, attribs: Vec<&'static str>) {
        self.attributes = attribs;
    }

}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server: {}\n", self.ldap_server)?;
        write!(f, "base dn: {}\n", self.base_dn)?;
        write!(f, "bind_dn: {}\n", self.bind_dn)?;
        write!(f, "auth_pass: {}", self.auth_pass)
    }
}
