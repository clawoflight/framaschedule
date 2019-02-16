pub mod framadate {
    use csv::Reader;
    use std::collections::HashMap;
    pub use std::io::Error as IoError;
    use std::io::ErrorKind as IoErrorKind;

    #[derive(Debug)]
    pub enum Response {
        Yes,
        No,
        IfNeedBe,
    }
    impl Response {
        pub fn from_str(s: &str) -> Result<Response, IoError> {
            if s == "Yes" {
                Ok(Response::Yes)
            } else if s == "No" {
                Ok(Response::No)
            } else if s == "Ifneedbe" {
                Ok(Response::IfNeedBe)
            } else {
                Err(IoError::new(
                    IoErrorKind::Other,
                    format!("Invalid framadate response: {}", s),
                ))
            }
        }
    }

    pub type TimePoint = String;
    pub type Name = String;
    pub type PollData = Vec<(TimePoint, HashMap<Name, Response>)>;

    pub fn read_data(file_name: &str) -> Result<PollData, IoError> {
        let mut data = Vec::new();
        let mut rdr = Reader::from_path(file_name)?;

        for h in rdr.headers()? {
            if h != "" {
                data.push((h.to_string(), HashMap::new()));
            }
        }
        for (i, r) in rdr.records().enumerate() {
            // Ignore the times, I don't need them TODO add them to the names of the days
            if i == 0 {
                continue;
            }

            let mut name = "";
            for (i, response) in r?.iter().enumerate() {
                // The first "response" is the participant name
                if i == 0 {
                    name = response;
                    continue;
                // The last field is empty
                } else if i - 1 == data.len() {
                    continue;
                }
                data[i - 1]
                    .1
                    .insert(name.to_string(), Response::from_str(response)?);
            }
        }

        Ok(data)
    }
}

pub mod scheduling {
    use crate::framadate::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::HashMap;

    pub type Schedule = Vec<(TimePoint, Name)>;
    fn occur(s: &Schedule, n: &str) -> usize {
        s.iter().filter(|&(_, name)| name == n).count()
    }

    pub type ScheduleWithCost = (f32, Schedule);
    pub type OrderedResults = (Option<ScheduleWithCost>, Option<ScheduleWithCost>);

    fn add_result(res: OrderedResults, new: ScheduleWithCost) -> OrderedResults {
        match res {
            (Some(r1), None) => {
                if r1.0 > new.0 {
                    (Some(new), Some(r1))
                } else {
                    (Some(r1), Some(new))
                }
            }
            (Some(r1), Some(r2)) => {
                if r1.0 > new.0 {
                    (Some(new), Some(r1))
                } else if r2.0 > new.0 {
                    (Some(r1), Some(new))
                } else {
                    (Some(r1), Some(r2))
                }
            }
            (None, None) => (Some(new), None),
            (None, Some(_)) => unreachable!(),
        }
    }

    pub fn print_schedule(s: ScheduleWithCost) {
        for (t, p) in s.1 {
            println!("{}:\t{}", t, p);
        }
        println!("\nCost: {}", s.0);
    }

    pub fn compute_all_schedules(data: &PollData) -> OrderedResults {
        let mut r = (None, None);
        compute_all_schedules_(data, vec![], &mut r);
        r
    }

    fn compute_all_schedules_(data: &PollData, cur_sched: Schedule, results: &mut OrderedResults) {
        // Allow early cutoff: don't assign people much more than necessary and calculate cost, but drop immediately
        let max_occur = data.len() / data[0].1.len() + 1;

        if cur_sched.len() == data.len() {
            *results = add_result(results.clone(), calc_schedule_cost(cur_sched, data))
        } else {
            let day = &data[cur_sched.len()];
            // TODO shuffle pairs here to randomize
            for (p, r) in &day.1 {
                if occur(&cur_sched, p) == max_occur {
                    continue;
                }
                match r {
                    Response::No => (),
                    _ => {
                        let mut new_sched = cur_sched.clone();
                        new_sched.push((day.0.to_string(), p.to_string()));
                        compute_all_schedules_(data, new_sched, results);
                    }
                }
            }
        }
    }

    fn calc_avg_distance_components(s: &Schedule) -> f32 {
        let mut last_seen = HashMap::new();
        let mut dsts = HashMap::new();
        for (i, (_, person)) in s.iter().enumerate() {
            let last_seen_i = last_seen.entry(person).or_insert(i);
            let dsts = dsts.entry(person).or_insert(Vec::new());
            let dst = i as f32 - *last_seen_i as f32;
            if dst > 0.0 {
                dsts.push(dst);
            }
            last_seen.insert(person, i);
        }

        let mut result = 0.0;
        for dsts in dsts.values() {
            let avg_dst: f32 = dsts.iter().sum();
            result += 1.0 / (avg_dst * avg_dst);
        }
        result
    }

    fn calc_ifneedbe_components(s: &Schedule, data: &PollData) -> f32 {
        let mut result = 0.0;
        for (i, (_, person)) in s.iter().enumerate() {
            if let Some(Response::IfNeedBe) = data[i].1.get(person) {
                result += 1.0;
            }
        }
        result
    }

    fn calc_schedule_cost(s: Schedule, data: &PollData) -> ScheduleWithCost {
        // TODO penalize ifneedbe, maybe by incrementing occurrences twice?
        let mut cost = 0.0;
        let mut person_occurences = HashMap::new();

        for (_, person) in &s {
            let occ = person_occurences.entry(person).or_insert(0);
            *occ += 1;
        }
        // TODO I could increase occurrences by 1 for every IfNeedBe here...

        for occ in person_occurences.values() {
            cost += (occ * occ) as f32;
        }
        cost += calc_avg_distance_components(&s);
        cost += calc_ifneedbe_components(&s, data);
        (cost, s)
    }
}
