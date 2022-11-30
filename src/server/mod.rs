use std::{env, fmt::Display};
mod config;
use config::Config;

#[derive(Default, Clone)]
pub struct Server {
    config: Config,
    ldap_server: String,
    ldap_domain: String,
    base_dn: String,
    bind_dn: String,
    auth_pass: String,
    filter: String,
    attributes: Vec<&'static str>,
}

pub fn configure_env(server: &mut Server, from_path: bool) {
    let path = env::var("CARGO_MANIFEST_DIR").ok();

    let config = Config::from_path(path, from_path);
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

    pub fn bind_dn(&self) -> String {
        self.bind_dn.clone()
    }

    pub fn auth_pass(&self) -> String {
        self.auth_pass.clone()
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
        self.ldap_server = self.config.get_var("LDAP_SERVER").unwrap();
        self.ldap_domain = self.config.get_var("LDAP_DOMAIN").unwrap();
        self.base_dn = self.config.get_var("LDAP_BASE_DN").unwrap();
        self.bind_dn = self.config.get_var("LDAP_BIND_DN").unwrap();
        self.auth_pass = self.config.get_var("LDAP_ADMIN_PASSWORD").unwrap();
    }

    pub fn set_filter(&mut self, filter: &str) -> &mut Self {
        self.filter = String::from(filter);
        self
    }

    pub fn set_attribs(&mut self, attribs: Vec<&'static str>) -> &mut Self {
        self.attributes = attribs;
        self
    }

}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "server: {}", self.ldap_server)?;
        writeln!(f, "domain: {}", self.ldap_domain)?;
        writeln!(f, "base dn: {}", self.base_dn)?;
        writeln!(f, "bind_dn: {}", self.bind_dn)?;
        writeln!(f, "auth_pass: {}", self.auth_pass)
    }
}
