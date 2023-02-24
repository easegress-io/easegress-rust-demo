use std::collections::HashMap;
use std::time::Duration;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct BlockRandom {
    block_ration: f64,
}

#[easegress_object]
impl Program for BlockRandom {
    fn new(_param: HashMap<String, String>) -> Self {
        Self {
            block_ration: 0.4,
        }
    }

    fn run(&self) -> i32 {
        if rand() > self.block_ration {
            return 0
        }

        response::resp_set_body("sold out.\n".as_bytes().to_vec());
        2
    }
}
