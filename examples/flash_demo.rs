use std::collections::HashMap;
use easegress_sdk::*;
use easegress_macros::easegress_object;

#[easegress_object]
struct AddHeaderAndSetBody {}

#[easegress_object]
impl Program for AddHeaderAndSetBody {
    fn new(_param: HashMap<String, String>) -> Self {
        Self {}
    }

    fn run(&self) -> i32 {
        let v = request::get_header("Foo".to_string());
        if v.len() > 10 {
            log(LogLevel::Warning, format!("The length of Foo is greater than 10"));
        }

        if v.len() > 0 {
            request::add_header("Wasm-Added".to_string(), v.clone());
        }

        request::add_header("Wasm-Added".to_string(), v.clone());
        request::set_body("I have a new body now".as_bytes().to_vec());
        0
    }
}
