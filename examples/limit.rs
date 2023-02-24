use std::collections::HashMap;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct Limit {
    block_ration: f64,
    max_permisssion: i32,
}

#[easegress_object]
impl Program for Limit {
    fn new(_param: HashMap<String, String>) -> Self {
        Self {
            block_ration: 0.4,
            max_permisssion: 3,
        }
    }

    fn run(&self) -> i32 {
        let id = request::get_header("Authorization".to_string());
        if cluster::get_string(format!("limit-id/{}", id)) == "true" {
            return 0
        }

        if cluster::count_key(format!("limit-id/")) < self.max_permisssion {
            if rand() > self.block_ration {
                cluster::put_string(format!("limit-id/{}", id), "true".to_string());
                return 0;
            }
        }

        response::resp_set_body("sold out.\n".as_bytes().to_vec());
        2
    }
}
