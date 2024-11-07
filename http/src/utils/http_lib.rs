pub mod http_helper {

    pub fn parse_request(req: &str) {
        let lines: Vec<&str> = req.split("\r\n").collect();
        let req_line = lines[0];
    }
}
