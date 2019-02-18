use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::BestSchedules;

fn main() -> Result<(), Box<Error>> {
    let data = framadate::read_data("/home/bennett/Downloads/Bild Feber-Mai.csv")?;

    match scheduling::compute_all_schedules(&data) {
        BestSchedules::Two(r1, r2) => {
            r1.print();
            r2.print();
        }
        BestSchedules::One(r1) => r1.print(),
        BestSchedules::None => eprintln!("No valid schedule exists!")
    }

    Ok(())
}
