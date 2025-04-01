use std::{error, fmt};


#[derive(Debug, PartialEq)]
pub enum ProxyError {
    NoJwtSpecified,
    InvalidJwt
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProxyError::NoJwtSpecified =>
                write!(f, "No jwt specified"),
                ProxyError::InvalidJwt =>
                write!(f, "Invalid jwt")
        }
    }
}

impl error::Error for ProxyError {

}
