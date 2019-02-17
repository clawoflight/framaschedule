use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;

fn main() -> Result<(), IoError> {
    let data = framadate::read_data("/home/bennett/Downloads/Bild Feber-Mai.csv")?;

    if let (Some(r1), Some(r2)) = scheduling::compute_all_schedules(&data) {
        scheduling::print_schedule(r1);
        println!("");
        scheduling::print_schedule(r2);
    }

    Ok(())
}
