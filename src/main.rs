use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::BestSchedules;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<Error>> {
    // TODO: try the macro version and see if I prefer it: https://docs.rs/clap/2.32.0/clap/
    let args = App::new("framaschedule")
        .about("Automatically find the best schedule based on poll data")
        .version(crate_version!())
        .author("Bennett Piater <bennett@piater.name>")
        .arg(
            Arg::with_name("POLL.csv")
                .help("The csv file exported from framadate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("csv")
                .help("output the best schedule in csv format")
                .long("export-csv")
                .value_name("output")
                .takes_value(true),
        )
        .get_matches();

    let data = framadate::read_data(args.value_of("POLL.csv").unwrap())?;

    let result = scheduling::compute_all_schedules(&data);

    if args.is_present("csv") {
        match result {
            BestSchedules::Two(r1, _) | BestSchedules::One(r1) => {
                r1.write_csv(args.value_of("csv").unwrap())?
            }
            _ => eprintln!("No valid schedule exists!"),
        }
    } else {
        match result {
            BestSchedules::Two(r1, r2) => {
                r1.print();
                r2.print();
            }
            BestSchedules::One(r1) => r1.print(),
            BestSchedules::None => eprintln!("No valid schedule exists!"),
        }
    }

    Ok(())
}
