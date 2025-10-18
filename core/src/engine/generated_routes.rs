// GENERATED FILE - DO NOT EDIT

#[allow(non_snake_case)]
mod module__home_maxniederwimmer_Code_Framework_core_src_engine__________example_main_rs {
    //'api'
    //import hashmap
    use std::collections::HashMap;
    
    
    // Handler for GET /main
    
    pub fn GET(_params: &HashMap<String, String>) -> String {
    
        "Hello from example main".to_string()
    
    }
}

use std::option::Option;
use std::boxed::Box;

pub type Handler = fn(&std::collections::HashMap<String, String>) -> String;

pub fn get_handler(route: &str, method: &str) -> Option<Handler> {
    match (route, method) {
        ("/main", "GET") => Some(module__home_maxniederwimmer_Code_Framework_core_src_engine__________example_main_rs::GET),
        _ => None,
    }
}
