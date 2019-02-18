pub use simple_error::SimpleError;
pub use std::collections::HashMap;
pub use std::error::Error;

#[derive(Debug)]
pub enum Response {
    Yes,
    No,
    IfNeedBe,
}

impl Response {
    pub fn from_str(s: &str) -> Result<Response, SimpleError> {
        match s {
            "Yes" => Ok(Response::Yes),
            "No" => Ok(Response::No),
            "Ifneedbe" | "IfNeedBe" => Ok(Response::IfNeedBe),
            _ => Err(SimpleError::new(format!(
                "Invalid framadate response string: {}",
                s
            ))),
        }
    }
}

pub type Slot = String;
pub type Name = String;
pub type PollData = Vec<PollColumn>;

#[derive(Debug)]
pub struct PollColumn {
    pub time: Slot,
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
