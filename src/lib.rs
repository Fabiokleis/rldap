pub mod server;
pub mod client;

#[cfg(test)]
mod test {

    #[test]
    fn try_to_load_envs() {
        use crate::server;
        let mut server = server::Server::default();
        server::configure_env(&mut server).expect("could not load environment variables!");
        println!("{}", server.ldap_server());
    }

    #[test]
    fn try_search_without_tls_connection() -> Result<(), ldap3::LdapError> {
        use ldap3::{self, Scope, SearchEntry};
        use crate::client;
        const PORT: u32 = 389;


        let entries = client::Request::new()
            .connect(PORT, false)?
            .bind()?
            .search(
                "(&(objectClass=posixAccount)(uid=pinguim))",
                vec!["uid", "sn", "mail", "userPassword"],
                Scope::Subtree
                )?
            .entries().unwrap();

        for entry in entries {
            println!("{:?}", SearchEntry::construct(entry))
        }
        Ok(())
    }

    #[test]
    fn try_search_with_tls_connection() -> Result<(), ldap3::LdapError> {
        use ldap3::{self, Scope, SearchEntry};
        use crate::client;
        const PORT: u32 = 636;


        let entries = client::Request::new()
            .connect(PORT, true)?
            .bind()?
            .search(
                "(&(objectClass=posixAccount)(uid=pinguim))",
                vec!["uid", "sn", "mail", "userPassword"],
                Scope::Subtree
                )?
            .entries().unwrap();

        for entry in entries {
            println!("{:?}", SearchEntry::construct(entry))
        }
        Ok(())
    }
}
    
