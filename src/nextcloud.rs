//! # Nextcloud import
//! This module can load data exported from [Nextcloud](https://nextcloud.org/).

use crate::data::*;
use chrono::NaiveDateTime;
use csv::Reader;
use scan_fmt::scan_fmt;

/// Reads data formatted like that from nextcloud.
///
/// If you are interested in that format specifically, check out `res/test/test_nextcloud.csv`
pub fn read_data(file_name: &str) -> Result<PollData, Box<dyn Error>> {
    let mut data = Vec::new();
    let mut rdr = Reader::from_path(file_name)?;

    for (i, r) in rdr.records().enumerate() {
        // skip description and end time columns (title column is the header)
        if i == 0 || i == 2 {
            continue;
        }
        // Parse the from time
        if i == 1 {
            // Skip "from" column
            for time in r?.iter().skip(1) {
                // TODO: attempt parsing the date and returning a short RFC string if possible?
                if let Ok(parsed) = NaiveDateTime::parse_from_str(time, "%a, %b %e, %Y %l:%M %p") {
                    data.push(PollColumn::new(&parsed.format("%Y-%m-%d").to_string()))
                } else {
                    data.push(PollColumn::new(time))
                }
            }
            continue;
        }

        let mut name = "";
        for (i, response) in r?.iter().take(data.len() + 1).enumerate() {
            // The first "response" is the participant name
            if i == 0 {
                name = response;
                continue;
            }
            let resp = Response::from_nextcloud_str(response)?;
            data[i - 1].responses.insert(name.to_owned(), resp);
        }
    }

    Ok(data)
}
