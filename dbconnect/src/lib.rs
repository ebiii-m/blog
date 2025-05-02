use once_cell::sync::OnceCell;


pub mod get_data;
pub mod set_data;
pub mod find;
pub mod group;

pub mod connect {
    #[derive(Debug)]
    pub struct DBConf {
        pub server: String,
        pub port: u32,
    }

    impl DBConf {
        pub fn new(server: String, port: u32) -> DBConf {
            DBConf {
                server: server,
                port: port,
            }
        }

        pub fn get_server(&self) -> String {
            format!("{}:{}", &self.server, &self.port)
        }
    }
}

static CONFIG: OnceCell<connect::DBConf> = OnceCell::new();

pub fn set_config(conf: connect::DBConf) -> Result<(), &'static str> {
    CONFIG
        .set(conf)
        .map_err(|_| "Config can only be set once!")
}

pub fn get_config() -> String {
    CONFIG.get().expect("Config not set!").get_server()
}

mod api_response {
    use serde_json::Value;
    use serde::{ Deserialize , Serialize };

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ApiResponse {
        pub status: String,
        pub message: Option<String>,
        pub rec_id: Option<String>,
        pub data: Option<Value>,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        
    }
}
