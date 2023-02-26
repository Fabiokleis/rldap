extern crate ldap3;
use ldap3::{Scope, SearchEntry};

pub mod server;
pub mod client;
use client::Request;

const PORT: u32 = 636;

fn main() -> Result<(), ldap3::LdapError> {
    let entries = Request::new()
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
