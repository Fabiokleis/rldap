extern crate ldap3;
use ldap3::{Scope, SearchEntry};

pub mod server;
pub mod client;
use server::Server;
use client::Request;


fn main() -> Result<(), ldap3::LdapError> {
    let mut server = Server::default();
    server::configure_env(&mut server, false);
    server.set_filter("(&(objectClass=posixAccount)(uid=pinguim))")
        .set_attribs(vec!["uid","sn", "mail", "userPassword"]);

    println!("{}", server);

    let mut request = Request::new();
    let conn = request.connect(format!("ldap://{}", server.ldap_server()))?;
    println!("{:?}", conn.entries());

    let bind = conn.bind(server.bind_dn().as_str(), server.auth_pass().as_str())?;
    println!("{:?}", bind.entries());

    let search = bind.search(server.base_dn().as_str(), Scope::Subtree, server.filter().as_str(), server.attribs())?;
    println!("{:?}", search.entries());

    let rs = search.entries().unwrap();

    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    println!("Hello, Ldap!");
    request.unbind()
}
