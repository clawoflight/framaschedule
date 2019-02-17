use crate::data::*;
use csv::Reader;

pub fn read_data(file_name: &str) -> Result<PollData, IoError> {
    let mut data = Vec::new();
    let mut rdr = Reader::from_path(file_name)?;

    // TODO transform date to DD.MM for my use-case
    for time in rdr.headers()? {
        if time != "" {
            data.push(PollColumn::new(time));
        }
    }
    for (i, r) in rdr.records().enumerate() {
        // Ignore the times, I don't need them
        // TODO add them to the names of the days - probably best to push the PollColumn here in that case
        if i == 0 {
            continue;
        }

        let mut name = "";
        for (i, response) in r?.iter().enumerate() {
            // The first "response" is the participant name
            if i == 0 {
                name = response;
                continue;
            // The last field is empty
            } else if i - 1 == data.len() {
                continue;
            }
            data[i - 1]
                .responses
                .insert(name.to_owned(), Response::from_str(response)?);
        }
    }

    Ok(data)
}
