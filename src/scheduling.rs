use crate::data::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

// TODO create proper datatypes

pub type Schedule = Vec<(TimePoint, Name)>;
fn occur(s: &Schedule, n: &str) -> usize {
    s.iter().filter(|&(_, name)| name == n).count()
}

pub type ScheduleWithCost = (f32, Schedule);
pub type OrderedResults = (Option<ScheduleWithCost>, Option<ScheduleWithCost>);

// TODO use vectors and keep all schedules with same score
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

// TODO add CSV export
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
    let mut person_occurrences = HashMap::new();

    for (_, person) in &s {
        let occ = person_occurrences.entry(person).or_insert(0);
        *occ += 1;
    }

    // TODO include occurrences in ScheduleWithCost to be able to print them
    for occ in person_occurrences.values() {
        cost += (occ * occ) as f32;
    }
    cost += calc_avg_distance_components(&s);
    cost += calc_ifneedbe_components(&s, data);
    (cost, s)
}
