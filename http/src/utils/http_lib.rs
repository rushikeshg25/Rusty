pub mod http_helper {
    use std::collections::HashMap;

    use http::types::http_types::http_types::{HttpMethods, HttpRequest};

    pub fn parse_request(req: &str) -> HttpRequest {
        let mut isbody: bool = false;
        let lines: Vec<&str> = req.split("\r\n").collect();
        let req_info: Vec<&str> = lines[0].split("/").map(|i| i.trim()).collect();
        let body: HashMap<&str, &str> = HashMap::new();
        for i in &req_info {
            println!("{}", i);
            if i.starts_with("}") {
                isbody = false;
            }
            if isbody {
                // let bodysplit=i.split("pat")
            }
            if i.starts_with("{") {
                isbody = true;
            }
        }

        return HttpRequest {
            host: req_info[1].to_string(),
            http_version: req_info[2].to_string(),
            method_type: match req_info[0] {
                "GET" => HttpMethods::GET,
                "PUT" => HttpMethods::PUT,
                "POST" => HttpMethods::POST,
                "DELETE" => HttpMethods::DELETE,
                _ => panic!("Invalid Request type"),
            },
        };
    }
}
