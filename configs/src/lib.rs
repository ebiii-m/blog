use blogs;
use api;
use render;

pub mod config_server {
    pub struct ServerConf {
        server_name: String,
        port: u32,
        base_dir: String,
    }

    impl ServerConf {
        pub fn new(server_name: String, port: u32, base_dir: String) -> ServerConf {
            ServerConf {
                server_name: server_name,
                port: port,
                base_dir: base_dir,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
