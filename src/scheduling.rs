use crate::data::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub time: TimePoint,
    pub name: Name,
}

impl ScheduleEntry {
    pub fn new(time: TimePoint, name: Name) -> ScheduleEntry {
        ScheduleEntry { time, name }
    }
}

pub type Schedule = Vec<ScheduleEntry>;

fn occur(s: &Schedule, n: &str) -> usize {
    s.iter().filter(|&e| e.name == n).count()
}

#[derive(Debug, Clone)]
pub struct EvaluatedSchedule {
    pub entries: Schedule,
    pub cost: f32,
    pub name_counts: Vec<(Name, usize)>,
}

impl EvaluatedSchedule {
    pub fn new(entries: Schedule, cost: f32) -> EvaluatedSchedule {
        EvaluatedSchedule {
            entries,
            cost,
            name_counts: Vec::new(),
        }
    }

    // TODO add CSV export
    pub fn print(&self) {
        for entry in &self.entries {
            println!("{}:\t{}", entry.time, entry.name);
        }
        println!("\nCost: {}", self.cost);
    }
}

//pub type ScheduleWithCost = (f32, Schedule);
pub type OrderedResults = (Option<EvaluatedSchedule>, Option<EvaluatedSchedule>);

// TODO use vectors and keep all schedules with same score
fn keep_best(res: OrderedResults, new: EvaluatedSchedule) -> OrderedResults {
    match res {
        (Some(r1), None) => {
            if r1.cost > new.cost {
                (Some(new), Some(r1))
            } else {
                (Some(r1), Some(new))
            }
        }
        (Some(r1), Some(r2)) => {
            if r1.cost > new.cost {
                (Some(new), Some(r1))
            } else if r2.cost > new.cost {
                (Some(r1), Some(new))
            } else {
                (Some(r1), Some(r2))
            }
        }
        (None, None) => (Some(new), None),
        (None, Some(_)) => unreachable!(),
    }
}

pub fn compute_all_schedules(data: &PollData) -> OrderedResults {
    let mut r = (None, None);
    compute_all_schedules_(data, vec![], &mut r);
    r
}

fn compute_all_schedules_(data: &PollData, cur_sched: Schedule, results: &mut OrderedResults) {
    // Allow early cutoff: don't assign people much more than necessary and calculate cost, but drop immediately
    let max_occur = data.len() / data[0].responses.len() + 1;

    if cur_sched.len() == data.len() {
        *results = keep_best(results.clone(), evaluate(cur_sched, data))
    } else {
        let day = &data[cur_sched.len()];
        // NOTE since the hash is not deterministic, this implicitly shuffles the names
        for (person, response) in &day.responses {
            if occur(&cur_sched, person) == max_occur {
                continue;
            }
            match response {
                Response::No => (),
                _ => {
                    let mut new_sched = cur_sched.clone();
                    new_sched.push(ScheduleEntry::new(day.time.to_owned(), person.to_owned()));
                    compute_all_schedules_(data, new_sched, results);
                }
            }
        }
    }
}

fn calc_avg_distance_components(s: &Schedule) -> f32 {
    let mut last_seen = HashMap::new();
    let mut dsts = HashMap::new();

    for (i, person) in s.iter().map(|e| &e.name).enumerate() {
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
    for (i, person) in s.iter().map(|e| &e.name).enumerate() {
        if let Some(Response::IfNeedBe) = data[i].responses.get(person) {
            result += 1.0;
        }
    }
    result
}

fn evaluate(s: Schedule, data: &PollData) -> EvaluatedSchedule {
    let mut cost = 0.0;
    let mut person_occurrences = HashMap::new();

    for person in s.iter().map(|e| &e.name) {
        let occ = person_occurrences.entry(person).or_insert(0);
        *occ += 1;
    }

    // TODO include occurrences in EvaluatedSchedule to be able to print them
    for occ in person_occurrences.values() {
        cost += (occ * occ) as f32;
    }
    cost += calc_avg_distance_components(&s);
    cost += calc_ifneedbe_components(&s, data);

    EvaluatedSchedule::new(s, cost)
}
