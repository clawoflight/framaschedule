use framaschedule::framadate;
use framaschedule::scheduling;
use framaschedule::scheduling::{BestSchedules, SchedulingOptions};

#[test]
fn known_schedule_cost() {
    let data = framadate::read_data("res/test/test_poll.csv").unwrap();
    if let BestSchedules::Two(r1, r2) =
        scheduling::compute_all_schedules(&data, &SchedulingOptions::default())
    {
        assert_eq!(4.0, r1.cost);
        assert_eq!(4.0, r2.cost);
    }
}

#[test]
fn impossible_schedule() {
    let data = framadate::read_data("res/test/test_impossible.csv").unwrap();
    match scheduling::compute_all_schedules(&data, &SchedulingOptions::default()) {
        BestSchedules::None => (),
        _ => panic!("Expected no solution to exist"),
    }
}

#[test]
fn impossible_schedule_forced() {
    let data = framadate::read_data("res/test/test_impossible.csv").unwrap();
    let opts = SchedulingOptions {
        ignore_empty_slots: true,
    };
    match scheduling::compute_all_schedules(&data, &opts) {
        BestSchedules::None => panic!("Expected a solution to exist"),
        _ => (),
    }
}

// TODO add tests for doodle
