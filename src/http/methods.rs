use crate::LioneError;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Method {
    Head,
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = LioneError;
    fn from_str(s: &str) -> Result<Self, LioneError> {
        match s.to_lowercase().as_str() {
            "head" => Ok(Self::Head),
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            "delete" => Ok(Self::Delete),
            _ => Err(LioneError::MethodNotFound(s.to_string())),
        }
    }
}
