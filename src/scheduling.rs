use crate::data::*;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub time: Slot,
    pub name: Name,
}

impl ScheduleEntry {
    fn new(time: Slot, name: Name) -> ScheduleEntry {
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
    fn new(entries: Schedule, cost: f32, name_counts: Vec<(Name, usize)>) -> EvaluatedSchedule {
        EvaluatedSchedule {
            entries,
            cost,
            name_counts,
        }
    }

    pub fn print(&self) {
        let mut counts = self.name_counts.clone();
        counts.sort();

        for entry in &self.entries {
            println!("{}:\t{}", entry.time, entry.name);
        }
        println!("\nCost: {}", self.cost);
        println!("\nStats:");
        for (name, count) in counts {
            println!("{}: {}", name, count)
        }
        println!();
    }

    pub fn write_csv(&self, path: &str) -> Result<(), Box<Error>> {
        let mut writer = csv::Writer::from_path(path)?;
        writer.write_record(&["slot", "name"])?;

        for entry in &self.entries {
            writer.write_record(&[&entry.time, &entry.name])?;
        }
        writer.flush()?;
        Ok(())
    }
}

//pub type BestSchedules = (Option<EvaluatedSchedule>, Option<EvaluatedSchedule>);
#[derive(Debug, Clone)]
pub enum BestSchedules {
    One(EvaluatedSchedule),
    Two(EvaluatedSchedule, EvaluatedSchedule),
    None,
}

fn keep_best(res: &BestSchedules, new: EvaluatedSchedule) -> BestSchedules {
    match res {
        BestSchedules::One(r1) => {
            if r1.cost > new.cost {
                BestSchedules::Two(new, r1.clone())
            } else {
                BestSchedules::Two(r1.clone(), new)
            }
        }
        BestSchedules::Two(r1, r2) => {
            if r1.cost > new.cost {
                BestSchedules::Two(new, r1.clone())
            } else if r2.cost > new.cost {
                BestSchedules::Two(r1.clone(), new)
            } else {
                BestSchedules::Two(r1.clone(), r2.clone())
            }
        }
        BestSchedules::None => BestSchedules::One(new),
    }
}

pub fn compute_all_schedules(data: &PollData) -> BestSchedules {
    let mut r = BestSchedules::None;
    compute_all_schedules_(data, vec![], &mut r);
    r
}

fn compute_all_schedules_(data: &PollData, cur_sched: Schedule, results: &mut BestSchedules) {
    // Allow early cutoff: don't assign people much more than necessary and calculate cost, but drop immediately
    let max_occur = data.len() / data[0].responses.len() + 1;

    if cur_sched.len() == data.len() {
        *results = keep_best(results, evaluate(cur_sched, data))
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
        let dst = (i - *last_seen_i) as f32;
        if dst > 0.0 {
            dsts.push(dst);
        }
        last_seen.insert(person, i);
    }

    let mut result = 0.0;
    for dsts in dsts.values() {
        let avg_dst: f32 = dsts.iter().sum();
        if avg_dst > 0.0 {
            result += 1.0 / (avg_dst * avg_dst);
        }
    }
    result
}

fn calc_ifneedbe_components(s: &mut Schedule, data: &PollData) -> f32 {
    let mut result = 0.0;
    for (i, person) in s.iter_mut().map(|e| &mut e.name).enumerate() {
        if let Some(Response::IfNeedBe) = data[i].responses.get(person) {
            result += 0.25;
            person.push('?');
        }
    }
    result
}

fn evaluate(mut s: Schedule, data: &PollData) -> EvaluatedSchedule {
    let mut cost = 0.0;
    let mut person_occurrences = HashMap::new();

    for person in s.iter().map(|e| &e.name) {
        let occ = person_occurrences.entry(person).or_insert(0);
        *occ += 1;
    }

    let mut occ_stats = Vec::new();
    for (person, occ) in person_occurrences {
        cost += (occ * occ) as f32;
        occ_stats.push((person.to_owned(), occ))
    }
    cost += calc_avg_distance_components(&s);
    cost += calc_ifneedbe_components(&mut s, data);

    EvaluatedSchedule::new(s, cost, occ_stats)
}
