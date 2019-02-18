use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::BestSchedules;

fn main() -> Result<(), Box<Error>> {
    let data = framadate::read_data("/home/bennett/Downloads/Bild Feber-Mai.csv")?;

    if let BestSchedules::Two(r1, r2) = scheduling::compute_all_schedules(&data) {
        r1.print();
        r2.print();
    }

    Ok(())
}
