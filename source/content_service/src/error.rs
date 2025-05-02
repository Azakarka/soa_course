use std::{error, fmt};


#[derive(Debug, PartialEq)]
pub enum ContentServiceError {
    WrongPassword,
    UserNotExists (String),
    InvalidUuid (String),
}

impl fmt::Display for ContentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentServiceError::WrongPassword =>
                        write!(f, "Wrong password"),
            ContentServiceError::UserNotExists(username) =>
                        write!(f, "User with name {username} doesn't exist"),
            ContentServiceError::InvalidUuid(exception) =>
                        write!(f, "Invalid uuid: {exception}"),
        }
    }
}

impl error::Error for ContentServiceError {

}
