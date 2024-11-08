pub mod http_helper {
    use http::types::http_types::http_types::{HttpMethods, HttpRequest};
    use serde_json::Value;
    use std::collections::HashMap;

    pub fn parse_request(req: &str) -> HttpRequest {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body_map: Option<HashMap<String, String>> = None;
        let mut lines = req.split("\r\n");
        let http_req_type: HttpMethods = if let Some(http_type) = lines.next() {
            if http_type.starts_with("GET") {
                HttpMethods::GET
            } else if http_type.starts_with("POST") {
                HttpMethods::POST
            } else if http_type.starts_with("PUT") {
                HttpMethods::PUT
            } else if http_type.starts_with("DELETE") {
                HttpMethods::DELETE
            } else {
                panic!("Invalid Request type");
            }
        } else {
            panic!("Request type missing");
        };

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        if let Some(content_length) = headers.get("Content-Length") {
            if let Ok(len) = content_length.parse::<usize>() {
                let body_str: String = lines.collect::<Vec<&str>>().join("\r\n");
                if body_str.len() >= len {
                    if let Ok(json_value) = serde_json::from_str::<Value>(&body_str) {
                        if let Some(json_obj) = json_value.as_object() {
                            let mut body_hashmap = HashMap::new();
                            for (key, value) in json_obj {
                                if let Some(value_str) = value.as_str() {
                                    body_hashmap.insert(key.clone(), value_str.to_string());
                                }
                            }
                            body_map = Some(body_hashmap);
                        }
                    }
                }
            }
        }

        return HttpRequest {
            method_type: http_req_type,
            body: body_map,
            headers: headers,
        };
    }
}
