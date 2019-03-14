use clap::{_clap_count_exprs, arg_enum, clap_app, crate_version, value_t};
use framaschedule::data::*;
use framaschedule::scheduling;
use framaschedule::scheduling::{BestSchedules, SchedulingOptions};
use framaschedule::{doodle, framadate};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Format {
        Framadate,
        Doodle
    }
}

fn main() -> Result<(), Box<Error>> {
    let args = clap_app!(framaschedule =>
    (version: crate_version!())
    (author: "Bennett Piater <bennett@piater.name>")
    (about: "Automatically find the best schedule fulfilling poll responses")
    (@arg csv: --("export-csv") [output] "Output the best schedule in csv format")
    (@arg ignore_empty: -F --("force-if-empty") "Ignore slots that cannot be filled")
    (@arg format: -f --format <format> +case_insensitive "The format of the input file - framadate or doodle")
    (@arg POLLDATA: +required "The csv file with the poll data")
    )
    .get_matches();

    let data_file = args.value_of("POLLDATA").unwrap();
    let data = match value_t!(args.value_of("format"), Format).unwrap_or_else(|_| Format::Framadate)
    {
        Format::Framadate => framadate::read_data(data_file)?,
        Format::Doodle => doodle::read_data(data_file)?,
    };

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
