//! # Poll data representation
//! This module describes the data structures for poll data.

pub use simple_error::SimpleError;
use std::collections::HashMap;
pub use std::error::Error;
use std::str::FromStr;

/// Represents a valid poll response.
///
/// IfNeedBe will be used as little as possible by the scheduling algorithm.
#[derive(Debug, Clone)]
pub enum Response {
    /// The respondent can take a slot
    Yes,
    /// The respondent cannot take a slot
    No,
    /// The respondent is able to take a slot if necessary
    IfNeedBe,
}

impl Response {
    pub fn from_doodle_str(s: &str) -> Result<Response, SimpleError> {
        match s {
            "OK" => Ok(Response::Yes),
            "(OK)" => Ok(Response::IfNeedBe),
            "" => Ok(Response::No),
            _ => Err(SimpleError::new(format!(
                "Invalid doodle response string: {}",
                s
            ))),
        }
    }
}

impl FromStr for Response {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Response, SimpleError> {
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

/// Marker type for string representation of slots
pub type Slot = String;
/// Marker type for string representation of names
pub type Name = String;
pub type PollData = Vec<PollColumn>;

/// Represents the poll data for one slot.
///
/// It contains everyone's responses for one time slot in a `HashMap` indexed by name.
#[derive(Debug, Clone)]
pub struct PollColumn {
    pub time: Slot,
    pub responses: HashMap<Name, Response>,
}

impl PollColumn {
    /// Create a new poll column for the given time.
    ///
    /// The responses hashmap is initialized as well.
    pub fn new(time: &str) -> PollColumn {
        PollColumn {
            time: time.to_owned(),
            responses: HashMap::new(),
        }
    }
}
