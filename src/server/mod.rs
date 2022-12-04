use std::{env, fmt::Display};
mod config;
use config::Config;

/// Server constructor
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

/// Load environment variables by CARGO_MANIFEST_DIR
///
/// in production the CARGO_MANIFEST_DIR are empty
pub fn configure_env(server: &mut Server) -> Result<(), std::io::Error> {
    let path = env::var("CARGO_MANIFEST_DIR").ok();

    let config = Config::from_path(path);
    server.set_config(config);
    server.load_env_variables()?;
    Ok(())
}

impl Server {

    /// Returns ldap server String (ip|fqdn)
    pub fn ldap_server(&self) -> String {
        self.ldap_server.clone()
    }

    /// Returns ldap server base dn (dc=example,dc=local)
    pub fn base_dn(&self) -> String {
        self.base_dn.clone()
    }

    /// Returns ldap server bind dn (cn=admin,dc=example,dc=local)
    pub fn bind_dn(&self) -> String {
        self.bind_dn.clone()
    }

    /// Returns ldap server admin password (admin_pass)
    pub fn auth_pass(&self) -> String {
        self.auth_pass.clone()
    }

    /// Returns ldap server filter that will be used to make searchs (uid=user)
    pub fn filter(&self) -> String {
        self.filter.clone()
    }

    /// Returns ldap server attributes that will be used to make searchs (vec!["uid", ...])
    pub fn attribs(&self) -> Vec<&'static str> {
        self.attributes.clone()
    }

    /// Set a new value to Config struct 
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Load environment variables by calling Config impls
    pub fn load_env_variables(&mut self) -> Result<(), std::io::Error> {
        self.config.init()?;
        self.ldap_server = self.config.get_var("LDAP_SERVER").unwrap();
        self.ldap_domain = self.config.get_var("LDAP_DOMAIN").unwrap();
        self.base_dn = self.config.get_var("LDAP_BASE_DN").unwrap();
        self.bind_dn = self.config.get_var("LDAP_BIND_DN").unwrap();
        self.auth_pass = self.config.get_var("LDAP_ADMIN_PASSWORD").unwrap();
        Ok(())
    }

    /// Set a new value to filter, returns self to chain functions
    pub fn set_filter(&mut self, filter: &str) -> &mut Self {
        self.filter = String::from(filter);
        self
    }

    /// Set a new value to attributes, reteurns self to chain functions
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
