extern crate ldap3;
use ldap3::{LdapConn, Scope, SearchEntry};

pub mod server;
use server::Server;


fn main() -> Result<(), ldap3::LdapError> {
    let mut server = Server::default();
    server::configure_env(&mut server, false);
    println!("{}", server);

    let mut ldap = LdapConn::new(server.ldap_server().as_str())?;

    server.set_filter("(&(objectClass=posixAccount)(uid=pinguim))");
    server.set_attribs(vec!["uid", "givenName", "sn", "mail", "userPassword", "*"]);

    let (rs, _res) = ldap.search(
        server.base_dn().as_str(),
        Scope::Subtree,
        server.filter().as_str(),
        server.attribs()
    )?.success()?;
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
  
    println!("Hello, Ldap!");

    Ok(ldap.unbind()?)
}
