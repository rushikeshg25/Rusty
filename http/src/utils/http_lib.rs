pub mod http_helper {
    use http::types::http_types::http_types::{httpMethods, httpRequest};

    pub fn parse_request(req: &str) {
        let lines: Vec<&str> = req.split("\r\n").collect();
        let req_info: Vec<&str> = lines[0].split("/").collect();
        for i in req_info {
            println!("{}", i);
        }
    }
}
