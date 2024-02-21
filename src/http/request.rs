use crate::{
    http::{body::Body, methods::Method},
    LioneError,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Body,
}

impl TryFrom<&[u8; 1024]> for Request {
    type Error = LioneError;

    fn try_from(buf: &[u8; 1024]) -> Result<Self, Self::Error> {
        if buf.is_empty() {
            return Err(LioneError::EmptyRequest);
        }

        let mut lines = buf.split(|&byte| byte == b'\n');
        let request_line = lines.next().ok_or(LioneError::EmptyRequest)?;
        let (method, uri) = parse_request_line(request_line)?;

        let mut headers = HashMap::new();
        let mut body = Body::None;
        for line in &mut lines {
            if line == b"\r" {
                break;
            }
            let (name, value) = parse_header_line(line)?;
            headers.insert(name, value);
        }

        if let Some(content_length) = headers.get("content-length") {
            let content_length = content_length
                .parse::<usize>()
                .map_err(|_| LioneError::EmptyRequest)?;
            let body_start = buf
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .ok_or(LioneError::EmptyRequest)?
                + 4;
            let body_end = body_start + content_length;
            let body_str = std::str::from_utf8(&buf[body_start..body_end])
                .map_err(|_| LioneError::EmptyRequest)?;
            let body_str = body_str.trim_end_matches('\0');
            body = Body::parse(
                body_str.trim().to_string(),
                headers.get("content-type").map(|x| x.as_str()),
            )?;
        }

        Ok(Self {
            method,
            path: uri.0,
            query: uri.1,
            headers,
            body,
        })
    }
}

fn parse_request_line(
    line: &[u8],
) -> Result<(Method, (String, HashMap<String, String>)), LioneError> {
    let line = std::str::from_utf8(line).map_err(|_| LioneError::TodoError)?;
    let mut parts = line.split_whitespace();
    let method: Method = parts
        .next()
        .ok_or(LioneError::TodoError)?
        .parse()
        .map_err(|_| LioneError::TodoError)?;
    let uri = parts.next().ok_or(LioneError::TodoError)?;
    let (path, query) = parse_uri(uri)?;
    Ok((method, (path, query)))
}

fn parse_uri(uri: &str) -> Result<(String, HashMap<String, String>), LioneError> {
    let mut parts = uri.splitn(2, '?');
    let path = parts
        .next()
        .ok_or(LioneError::TodoError)?
        .trim()
        .to_string();
    let query = parts.next().map_or_else(HashMap::new, |q| parse_query(q));
    Ok((path, query))
}

//Parses a URL query string into a 'HashMap' of key-value pairs.
fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split("&")
        .filter_map(|pair| {
            let mut pair = pair.splitn(2, "=");
            let key = pair.next()?.trim().to_lowercase();
            let value = pair.next()?.trim().to_string();
            Some((key, value))
        })
        .collect()
}

//Parses a single header line into a tugle of header name and value
fn parse_header_line(line: &[u8]) -> Result<(String, String), LioneError> {
    let line = std::str::from_utf8(line).map_err(|_| LioneError::TodoError)?;
    let mut parts = line.splitn(2, ":");
    let name = parts
        .next()
        .ok_or(LioneError::TodoError)?
        .trim()
        .to_lowercase();
    let value = parts
        .next()
        .ok_or(LioneError::TodoError)?
        .trim()
        .to_lowercase();
    Ok((name, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer_from_test_data(test_data: &[u8]) -> [u8; 1024] {
        let mut buffer = [0; 1024];
        buffer[..test_data.len()].copy_from_slice(test_data);
        buffer
    }

    #[test]
    fn test_request_get_one() {
        let request_data = b"GET /path HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length:  17\r\n\r\n{\"key\": \"value\"}";
        let request: Result<Request, _> =
            Request::try_from(&create_buffer_from_test_data(request_data));
        assert!(request.is_ok());
        let request = request.unwrap();
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/path");
        assert_eq!(
            request.headers.get("content-type").unwrap(),
            "application/json"
        );
        let body_json = Body::parse(r#"{"key": "value"}"#.to_string(), Some("application/json"));
        assert!(matches!(request.body, body_json));
    }
}
