pub mod http_helper {
    use http::types::http_types::http_types::{httpMethods, httpRequest};

    pub fn parse_request(req: &str) -> httpRequest {
        let lines: Vec<&str> = req.split("\r\n").collect();
        let req_info: Vec<&str> = lines[0].split("/").map(|i| i.trim()).collect();
        // for i in req_info {
        //     println!("{}", i);
        // }
        return httpRequest {
            host: req_info[1].to_string(),
            http_version: req_info[2].to_string(),
            method_type: match req_info[0] {
                "GET" => httpMethods::GET,
                "PUT" => httpMethods::PUT,
                "POST" => httpMethods::POST,
                "DELETE" => httpMethods::DELETE,
                _ => panic!("Invalid Request type"),
            },
        };
    }
}
