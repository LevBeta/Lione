use crate::LioneError;
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Body {
    None,
    Text(String),
    Json(Value),
}

impl Body {
    pub fn parse(body: String, content_type: Option<&str>) -> Result<Self, LioneError> {
        match content_type {
            Some(content_type) if content_type.eq_ignore_ascii_case("application/json") => {
                Self::parse_json(body)
            }
            _ => Ok(Self::Text(body)),
        }
    }

    fn parse_json(body: String) -> Result<Self, LioneError> {
        serde_json::from_str(&body)
            .map(Self::Json)
            .map_err(LioneError::from)
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Text(body) => write!(f, "{body}"),
            Self::Json(body) => write!(f, "{body}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let body = "This is a text".to_string();
        let result = Body::parse(body.clone(), None);
        assert_eq!(result, Ok(Body::Text(body)));
    }

    #[test]
    fn test_parse_json() {
        let body = r#"{"key": "value"}"#.to_string();
        let result = Body::parse(body.clone(), Some("application/json"));
        assert!(matches!(result, Ok(Body::Json(_))));
    }

    #[test]
    fn test_parse_invalid_json() {
        let body = r#"{key: "value"}"#.to_string();
        let result = Body::parse(body.clone(), Some("application/json"));
        assert!(matches!(result, Err(_)));
    }
}
