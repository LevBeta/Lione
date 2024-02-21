#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
}

impl StatusCode {
    pub fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "200 OK".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
            StatusCode::BadRequest => "400 Bad Request".to_string(),
        }
    }
}
