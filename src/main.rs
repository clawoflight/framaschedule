use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;

fn main() -> Result<(), IoError> {
    let data = framadate::read_data("/home/bennett/Downloads/Bild Feber-Mai.csv")?;

    if let (Some(r1), Some(r2)) = scheduling::compute_all_schedules(&data) {
        r1.print();
        println!("");
        r2.print();
    }

    Ok(())
}
