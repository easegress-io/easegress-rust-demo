use std::collections::HashMap;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct Luck {
    block_ration: f64,
}

#[easegress_object]
impl Program for Luck {
    fn new(_param: HashMap<String, String>) -> Self {
        Self {
            block_ration: 0.4,
        }
    }

    fn run(&self) -> i32 {
        let id = request::get_header("Authorization".to_string());
        if cluster::get_string(format!("uid/{}", id)) == "true" {
            return 0
        }

        if rand() > self.block_ration {
            cluster::put_string(format!("uid/{}", id), "true".to_string());
            return 0
        }

        response::resp_set_body("sold out.\n".as_bytes().to_vec());
        2
    }
}
