//! # Framadate import
//! This module can load data exported from [Framadate](https://framadate.org/).

use crate::data::*;
use csv::Reader;

/// Reads data formatted like that from Framadate.
///
/// If you are interested in that format specifically, check out `res/test/test_poll.csv`
pub fn read_data(file_name: &str) -> Result<PollData, Box<Error>> {
    let mut data: PollData = Vec::new();
    let mut rdr = Reader::from_path(file_name)?;

    // Parse the top-level unit of time (always months AFAICT)
    for time in rdr.headers()?.iter().skip(1) {
        match time {
            // Only new days are included in the csv...
            "" => data.push(data[data.len() - 1].clone()),
            _ => data.push(PollColumn::new(time)),
        }
    }
    for r in rdr.records() {
        let r = r?;
        // Add day and time if they exist - checking for the absence of a name
        if &r[0] == "" {
            for (time, poll_column) in r.iter().skip(1).zip(&mut data) {
                poll_column.time += &format!(" {}", time);
            }
            continue;
        }

        let mut name = "";
        for (i, response) in r.iter().enumerate() {
            // The first "response" is the participant name
            if i == 0 {
                name = response;
                continue;
            }
            let resp = Response::from_doodle_str(response)?;
            data[i - 1].responses.insert(name.to_owned(), resp);
        }
    }

    Ok(data)
}
