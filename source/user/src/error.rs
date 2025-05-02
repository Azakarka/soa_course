use std::{error, fmt};


#[derive(Debug, PartialEq)]
pub enum UserServiceError {
    WrongPassword,
    UserNotExists (String)
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserServiceError::WrongPassword =>
                write!(f, "Wrong password"),
            UserServiceError::UserNotExists(username) =>
                write!(f, "User with name {username} doesn't exist")
        }
    }
}

impl error::Error for UserServiceError {

}
