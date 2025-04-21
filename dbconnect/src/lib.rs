use get_data;
use set_data;
use find;
use group;

pub mod connect {
    pub struct dbconf {
        server: String,
        port: u32,
    }

    impl dbconf {
        pub fn new(server: String, port: u32) -> dbconf {
            dbconf {
                server: server,
                port: port,
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
