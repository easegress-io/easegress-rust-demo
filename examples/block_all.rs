use std::collections::HashMap;
use std::time::Duration;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct BlockALL {
    start_time: u128,
}

#[easegress_object]
impl Program for BlockALL {
    fn new(_param: HashMap<String, String>) -> Self {
        Self {
            start_time: Duration::from_millis(1691424000000).as_millis(),
        }
    }

    fn run(&self) -> i32 {
        let now = get_unix_time_in_ms() as u128;
        if now < self.start_time {
            response::resp_set_body("not start yet.\n".as_bytes().to_vec());
            return 1
        }
        0
    }
}
