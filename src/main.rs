use clap::{clap_app, crate_version};
use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::{BestSchedules, SchedulingOptions};

fn main() -> Result<(), Box<Error>> {
    let args = clap_app!(framaschedule =>
    (version: crate_version!())
    (author: "Bennett Piater <bennett@piater.name>")
    (about: "Automatically find the best schedule fulfilling poll responses")
    (@arg POLLDATA: +required "The csv file exported from framadate")
    (@arg csv: --("export-csv") [output] "Output the best schedule in csv format")
    (@arg ignore_empty: -f --("force-if-empty") "Ignore slots that cannot be filled")
    )
    .get_matches();

    let data = framadate::read_data(args.value_of("POLLDATA").unwrap())?;

    let options = SchedulingOptions {
        ignore_empty_slots: args.is_present("ignore_empty"),
    };
    let result = scheduling::compute_all_schedules(&data, &options);

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
