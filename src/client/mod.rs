use ldap3::{LdapConn, ResultEntry, Scope, LdapError};

trait State {
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

#[derive(Clone)]
struct Connect;
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

#[derive(Clone)]
struct Bind;
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

#[derive(Clone)]
struct Search;
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

#[derive(Default)]
pub struct Request {
    state: Option<Box<dyn State>>,
    connection: Option<Box::<LdapConn>>,
    entries: Vec<ResultEntry>,
}


impl Request {
    pub fn new() -> Self {
        Request { 
            state: Some(Box::new(Connect)), 
            connection: None,
            entries: vec![],
        }
    }

    pub fn connect(&mut self, ldap_server: String) -> Result<&mut Self, LdapError> {
        if let Some(s) = self.state.take() {
            self.state = Some(s.req_connection())
        }
        self.connection = Some(Box::new(LdapConn::new(ldap_server.as_str())?));
        Ok(self)
    }

    pub fn unbind(&mut self) -> Result<(), LdapError> {
        self.connection.as_mut().unwrap().unbind()
    }

    pub fn bind(&mut self, bind_dn: &str, bind_pw: &str) -> Result<&mut Self, LdapError> {
        if let Some(s) = self.state.take() {
            self.state = Some(s.req_bind())
        }
        LdapConn::simple_bind(self.connection.as_mut().unwrap(), bind_dn, bind_pw)?;
        Ok(self)
    }

    pub fn search(&mut self,
        base_dn: &str,
        scope: Scope,
        filter: &str,
        attribs: Vec<&str>
        ) -> Result<&mut Self, LdapError> {

        if let Some(s) = self.state.take() {
            self.state = Some(s.req_search())
        }
        self.entries = self.connection.as_mut().map(|conn| 
            conn.search(
                base_dn,
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
