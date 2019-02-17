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
        match s {
            "Yes" => Ok(Response::Yes),
            "No" => Ok(Response::No),
            "Ifneedbe" | "IfNeedBe" => Ok(Response::IfNeedBe),
            _ => Err(IoError::new(
                IoErrorKind::Other,
                format!("Invalid framadate response: {}", s),
            )),
        }
    }
}

pub type TimePoint = String;
pub type Name = String;
pub type PollData = Vec<PollColumn>;

#[derive(Debug)]
pub struct PollColumn {
    pub time: TimePoint,
    pub responses: HashMap<Name, Response>,
}

impl PollColumn {
    pub fn new(time: &str) -> PollColumn {
        PollColumn {
            time: time.to_owned(),
            responses: HashMap::new(),
        }
    }
}
