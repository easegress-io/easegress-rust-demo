use std::collections::HashMap;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct Reuse {
    block_ration: f64,
    max_permisssion: i32,
}

#[easegress_object]
impl Program for Reuse {
    fn new(param: HashMap<String, String>) -> Self {
        let mut block_ration: f64 = 0.0;
        let mut max_permission = 0;

        if param.contains_key("blockRatio") {
            let val = param.get("blockRatio").unwrap();
            block_ration = val.to_string().parse::<f64>().unwrap();
        }

        if param.contains_key("maxPermission") {
            let val = param.get("maxPermission").unwrap();
            max_permission = val.to_string().parse::<i32>().unwrap();
        }

        Self {
            block_ration: block_ration,
            max_permisssion: max_permission,
        }
    }

    fn run(&self) -> i32 {
        let id = request::get_header("Authorization".to_string());
        if cluster::get_string(format!("limit-id/{}", id)).trim_end_matches(char::from(0)) == "true" {
            return 0;
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
