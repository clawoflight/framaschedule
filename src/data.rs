pub use std::collections::HashMap;
pub use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

#[derive(Debug)]
pub enum Response {
    Yes,
    No,
    IfNeedBe,
}
impl Response {
    pub fn from_str(s: &str) -> Result<Response, IoError> {
        if s == "Yes" {
            Ok(Response::Yes)
        } else if s == "No" {
            Ok(Response::No)
        } else if s == "Ifneedbe" {
            Ok(Response::IfNeedBe)
        } else {
            Err(IoError::new(
                IoErrorKind::Other,
                format!("Invalid framadate response: {}", s),
            ))
        }
    }
}

pub type TimePoint = String;
pub type Name = String;
pub type PollData = Vec<(TimePoint, HashMap<Name, Response>)>;
