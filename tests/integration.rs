use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::BestSchedules;

#[test]
fn known_schedule_cost() {
    let data = framadate::read_data("res/test/test_poll.csv").unwrap();
    if let BestSchedules::Two(r1, r2) = scheduling::compute_all_schedules(&data) {
        assert_eq!(4.0, r1.cost);
        assert_eq!(4.0, r2.cost);
    }
}
