use crate::{
    http::{body::Body, statuscode::StatusCode}
};
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone)]
pub struct Response {
    pub statuscode: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Body,
}

impl Response {
    pub fn new() -> Self {
        Self {
            body: Body::None,
            statuscode: StatusCode::Ok,
            headers: HashMap::new(),
        }
    }

    pub fn text(&mut self, body: &str) -> Self {
        Self::new()
            .header("Content-type", "text/plain")
            .body(Body::Text(body.to_string()))
    }

    pub fn status(&mut self, code: StatusCode) -> Self {
        self.statuscode = code;
        self.clone()
    }

    pub fn body(&mut self, body: Body) -> Self {
        self.body = body;
        self.clone()
    }

    pub fn header(&mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());
        self.clone()
    }

    //PRE-MADE

    pub fn not_found() -> Self {
        Response::new()
            .text("Not found")
            .status(StatusCode::NotFound)
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = String::new();
        write!(response, "HTTP/1.1 {}\r\n", self.statuscode.to_string()).unwrap();
        for (name, value) in &self.headers {
            write!(response, "{}: {}\r\n", name, value).unwrap();
        }
        write!(response, "\r\n").unwrap();
        response.push_str(&self.body.to_string());
        response
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//    fn test_response_string() {
//        let response = Response::new().text("Hi from Lione");
//        assert_eq!(response.to_string(), "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHi from Lione".to_string());
//        println!("{:?}", response.to_string());
//    }
//}
