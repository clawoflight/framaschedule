use framaschedule::data::*;
use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::BestSchedules;

fn main() -> Result<(), IoError> {
    let data = framadate::read_data("/home/bennett/Downloads/Bild Feber-Mai.csv")?;

    if let BestSchedules::Two(r1, r2) = scheduling::compute_all_schedules(&data) {
        r1[0].print();
        println!("\n... and {} equivalent options\n", r1.len());
        r2[0].print();
        println!("\n... and {} equivalent options", r2.len());
        for sched in r1 {
            sched.print();
        }
    }

    Ok(())
}
