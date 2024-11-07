use std::collections::HashMap;

pub mod http_types {
    use super::*;
    #[derive(Debug)]
    pub struct http_request {
        host: String,
        connection: String,
        sec_ch_ua: String,
        sec_ch_ua_platform: String,
        dnt: String,
        user_agent: String,
    }

    pub fn response_status_codes() -> HashMap<i32, &'static str> {
        return HashMap::from([(200, "OK"), (404, "Not Found"), (501, "Not Implemented")]);
    }

    pub struct http_response {}
}
