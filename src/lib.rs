pub mod server;
pub mod client;

#[cfg(test)]
mod test {

    #[test]
    fn try_to_load_envs() {
        use crate::server;
        let mut server = server::Server::default();
        server::configure_env(&mut server).expect("could not load environment variables!");
    }
}
    
