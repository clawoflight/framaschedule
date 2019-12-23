//! # Framadate import
//! This module can load data exported from [Framadate](https://framadate.org/).

use crate::data::*;
use csv::Reader;
use scan_fmt::scan_fmt;

/// Reads data formatted like that from Framadate.
///
/// If you are interested in that format specifically, check out `res/test/test_poll.csv`
pub fn read_data(file_name: &str) -> Result<PollData, Box<dyn Error>> {
    let mut data = Vec::new();
    let mut rdr = Reader::from_path(file_name)?;

    for time in rdr.headers()? {
        if time != "" {
            let (month, day) = scan_fmt!(time, "{*d}-{d}-{d}", i32, i32)?;
            data.push(PollColumn::new(&format!(
                "{:02}.{:02}.",
                day,
                month
            )));
        }
    }
    for (i, r) in rdr.records().enumerate() {
        // Parse the times
        if i == 0 {
            // Skip empty name, ignore trailing empty column
            for (time, poll_column) in r?.iter().skip(1).take(data.len()).zip(&mut data) {
                poll_column.time += &format!(" {}", time);
            }
            continue;
        }

        let mut name = "";
        // Ignore empty trailing column
        for (i, response) in r?.iter().take(data.len() + 1).enumerate() {
            // The first "response" is the participant name
            if i == 0 {
                name = response;
                continue;
            }
            let resp: Response = response.parse()?;
            data[i - 1].responses.insert(name.to_owned(), resp);
        }
    }

    Ok(data)
}
