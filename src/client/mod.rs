use ldap3::{LdapConn, ResultEntry, Scope, LdapError};
use crate::server::{self, Server};

/// State design pattern to Request
pub trait State {
    fn req_connection(self: Box<Self>) -> Box<dyn State>;
    fn req_bind(self: Box<Self>) -> Box<dyn State>;
    fn req_search(self: Box<Self>) -> Box<dyn State>;
    fn get_entries(self: Box<Self>, _request: &Request) -> Option<Vec<ResultEntry>> {
       Some(vec![])
    }
    fn clone_dyn(&self) -> Box<dyn State>;
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

/// Connection State to LdapConn
#[derive(Clone)]
pub struct Connect;
impl State for Connect {
    fn req_connection(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn req_bind(self: Box<Self>) -> Box<dyn State> {
        Box::new(Bind {})
    }

    fn req_search(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn clone_dyn(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

/// Bind State to LdapConn
#[derive(Clone)]
pub struct Bind;
impl State for Bind {
    fn req_connection(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn req_bind(self: Box<Self>) -> Box<dyn State> {
        self 
    }

    fn req_search(self: Box<Self>) -> Box<dyn State> {
        Box::new(Search {})
    }

    fn clone_dyn(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

/// Search State to LdapConn
#[derive(Clone)]
pub struct Search;
impl State for Search {
    fn req_connection(self: Box<Self>) -> Box<dyn State> {
        Box::new(Connect {})
    }

    fn req_bind(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn req_search(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_entries(self: Box<Self>, request: &Request) -> Option<Vec<ResultEntry>> {
        let mut result: Vec<ResultEntry> = vec![];
        for entry in request.entries.iter() {
            result.push(entry.clone());
        }
        Some(result)
    } 

    fn clone_dyn(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

}


/// Request constructor
#[derive(Default)]
pub struct Request {
    state: Option<Box<dyn State>>,
    connection: Option<Box::<LdapConn>>,
    server: Server,
    entries: Vec<ResultEntry>,
}

impl Request {
    pub fn new() -> Self {
        Request { 
            state: Some(Box::new(Connect)), 
            connection: None,
            server: Server::default(),
            entries: vec![],
        }
    }

    /// Create a connection with LdapConn by calling LdapConn constructor
    ///
    /// Stores him on heap inside a Box smart pointer
    ///
    /// Update state to Connect
    pub fn connect(&mut self) -> Result<&mut Self, LdapError> {
        server::configure_env(&mut self.server)
            .expect("ERROR: Could not load environment variables from .env!!!");
        if let Some(s) = self.state.take() {
            self.state = Some(s.req_connection())
        }
        let server_uri = &self.server.ldap_server().as_str();
        self.connection = Some(
            Box::new(LdapConn::new(server_uri)?)
        );
        Ok(self)
    }

    /// Close the connection with LdapConn 
    pub fn unbind(&mut self) -> Result<(), LdapError> {
        self.connection.as_mut().unwrap().unbind()
    }

    /// Do bind on LdapConn by calling simple_bind
    ///
    /// Update state to Bind
    pub fn bind(&mut self) -> Result<&mut Self, LdapError> {
        if let Some(s) = self.state.take() {
            self.state = Some(s.req_bind())
        }
        LdapConn::simple_bind(
            self.connection.as_mut().unwrap(),
            self.server.bind_dn().as_str(), 
            self.server.auth_pass().as_str()
        )?;
        Ok(self)
    }

    /// Do search by calling search method of LdapConn
    ///
    /// Expects multiple arguments to perform filtered searchs
    ///
    /// Update state to Search
    pub fn search(&mut self,
        filter: &str,
        attribs: Vec<&str>,
        scope: Scope
        ) -> Result<&mut Self, LdapError> {

        if let Some(s) = self.state.take() {
            self.state = Some(s.req_search())
        }
        self.entries = self.connection.as_mut().map(|conn| 
            conn.search(
                self.server.base_dn().as_str(),
                scope,
                filter,
                attribs))
            .unwrap()?.success()?.0;
        Ok(self)
    }

    pub fn entries(&mut self) -> Option<Vec<ResultEntry>> {
        self.state.clone().unwrap().get_entries(self)
    }
}
