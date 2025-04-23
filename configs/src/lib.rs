pub mod blogs;
pub mod api;
pub mod render;
use serde_json::json;
use actix_web::web;

pub mod config_server {
    pub struct ServerConf {
        server_name: String,
        port: u32,
        base_dir: String,
    }

    pub struct Template {
        title: String,
        date: String,
        content: json,
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

    impl Template {
        pub fn new(title: String, date: String, content: json) -> Template {
            Template {
                title: title,
                date: date,
                content: content,
            }
        }
    }
}

pub mod blog_config {
    pub fn config(cfg: web::ServiceConfig) {
        cfg.service(blogs::blog_conf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
