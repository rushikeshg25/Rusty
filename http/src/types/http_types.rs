use std::collections::HashMap;

pub mod http_types {
    use super::*;

    pub struct HttpRequest {
        pub method_type: HttpMethods,
        pub host: String,
        pub http_version: String,
    }

    pub fn response_status_codes() -> HashMap<i32, &'static str> {
        return HashMap::from([(200, "OK"), (404, "Not Found"), (501, "Not Implemented")]);
    }

    pub enum HttpMethods {
        GET,
        POST,
        PUT,
        DELETE,
    }

    pub struct HttpResponse {}
}
