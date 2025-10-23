// GENERATED FILE - DO NOT EDIT

#[allow(non_snake_case)]
mod module__home_maxniederwimmer_Code_Framework_core_src_engine_______example_main_rs {
    mod __orig {
        //'api'
        // import hashmap
        use std::collections::HashMap;
        
        pub(crate) fn GET(_params: HashMap<String, String>) -> (String, u16) {
            ("Hello from GET".to_string(), 500)
        }
    }
    // wrapper for GET that adapts (String,u16) -> Response
    pub fn GET(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::GET(params.clone());
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
}

use std::option::Option;

pub type Handler = fn(&std::collections::HashMap<String, String>) -> super::Response;

pub fn get_handler(route: &str, method: &str) -> Option<Handler> {
    match (route, method) {
        ("/main", "GET") => Some(module__home_maxniederwimmer_Code_Framework_core_src_engine_______example_main_rs::GET),
        _ => None,
    }
}
