pub mod get_data;
pub mod set_data;
pub mod find;
pub mod group;

pub mod connect {
    pub struct DBConf {
        server: String,
        port: u32,
    }

    impl DBConf {
        pub fn new(server: String, port: u32) -> DBConf {
            DBConf {
                server: server,
                port: port,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        
    }
}
