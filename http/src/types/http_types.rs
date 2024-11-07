use std::collections::HashMap;

pub mod http_types {
    use super::*;

    pub struct httpRequest {
        method_type: httpMethods,
        host: String,
        http_version: String,
    }

    pub fn response_status_codes() -> HashMap<i32, &'static str> {
        return HashMap::from([(200, "OK"), (404, "Not Found"), (501, "Not Implemented")]);
    }

    pub enum httpMethods {
        GET,
        POST,
        PUT,
        DELETE,
    }

    pub struct http_response {}
}
