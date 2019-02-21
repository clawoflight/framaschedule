use crate::data::*;
use scoped_threadpool::Pool;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct SchedulingOptions {
    pub ignore_empty_slots: bool,
}

impl SchedulingOptions {
    pub fn default() -> SchedulingOptions {
        SchedulingOptions {
            ignore_empty_slots: false,
        }
    }
}

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

fn occur(s: &[ScheduleEntry], n: &str) -> usize {
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

#[derive(Debug, Clone)]
pub enum BestSchedules {
    One(EvaluatedSchedule),
    Two(EvaluatedSchedule, EvaluatedSchedule),
    None,
}

impl BestSchedules {
    // NOTE Maybe this could implement the trait for `+`?
    fn add(best: &BestSchedules, new: EvaluatedSchedule) -> BestSchedules {
        match best {
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

    fn merge(r1: BestSchedules, r2: BestSchedules) -> BestSchedules {
        match r1 {
            BestSchedules::None => r2,
            BestSchedules::One(b1) => BestSchedules::add(&r2, b1),
            BestSchedules::Two(b1, b2) => BestSchedules::add(&BestSchedules::add(&r2, b1), b2),
        }
    }
}

pub fn compute_all_schedules(data: &[PollColumn], opts: &SchedulingOptions) -> BestSchedules {
    let first_day = &data[0];
    // We are CPU-bound, so don't attempt hyper-threading
    let mut pool = Pool::new(num_cpus::get_physical() as u32);

    // Parallelize over the first day: one thread for the best solution starting with each name
    let mut results = vec![BestSchedules::None; first_day.responses.len()];

    // Using a scoped threadpool allows passing in non-static references.
    // I need that because I know that the lifetime of the thread closures will not exceed that of this function.
    pool.scoped(|scoped| {
        // zip with results to avoid mutably indexing, which would confuse the borrow checker:
        // It *looks* like multiple things depend on the vector, but they never actually collide
        // (each thread gets it's own element)
        for ((person, response), result) in first_day.responses.iter().zip(results.iter_mut()) {
            match response {
                Response::No => {
                    if opts.ignore_empty_slots {
                        let starting_sched = vec![ScheduleEntry::new(
                            first_day.time.to_owned(),
                            "??".to_string(),
                        )];
                        scoped.execute(move || {
                            compute_all_schedules_(data, opts, starting_sched, result)
                        });
                    }
                }
                _ => {
                    let starting_sched = vec![ScheduleEntry::new(
                        first_day.time.to_owned(),
                        person.to_owned(),
                    )];
                    scoped.execute(move || {
                        compute_all_schedules_(data, opts, starting_sched, result)
                    });
                }
            }
        }
    });

    // Combine partial solutions
    results.into_iter().fold(BestSchedules::None, |best, next| {
        BestSchedules::merge(best, next)
    })
}

// Alternative implementation:
// currently requires library features - wait for stabilization of `yield`
//
// - build a schedule generator
// - map to best schedule and fold
// - collect the result into a var
// This is at the very least an interesting academic exercise,
// but the separation of concerns would also be more readable.
// However, that would be hard to parallelize in divide-and-conquer
// -- unless I modify the generator to begin with a specific name, which would work well.
//

fn compute_all_schedules_(
    data: &[PollColumn],
    opts: &SchedulingOptions,
    cur_sched: Schedule,
    results: &mut BestSchedules,
) {
    // Allow early cutoff: don't assign people much more than necessary and calculate cost, but drop immediately
    let max_occur = data.len() / data[0].responses.len() + 1;

    if cur_sched.len() == data.len() {
        *results = BestSchedules::add(results, evaluate(cur_sched, data))
    } else {
        let day = &data[cur_sched.len()];
        // NOTE since the hash is not deterministic, this implicitly shuffles the names
        for (person, response) in &day.responses {
            // TODO this branch might have to be skipped if opts.ignore_empty_slots is set
            if occur(&cur_sched, person) == max_occur {
                continue;
            }
            match response {
                Response::No => {
                    if opts.ignore_empty_slots {
                        let mut new_sched = cur_sched.clone();
                        new_sched.push(ScheduleEntry::new(day.time.to_owned(), "??".to_string()));
                        compute_all_schedules_(data, opts, new_sched, results);
                    }
                }
                _ => {
                    let mut new_sched = cur_sched.clone();
                    new_sched.push(ScheduleEntry::new(day.time.to_owned(), person.to_owned()));
                    compute_all_schedules_(data, opts, new_sched, results);
                }
            }
        }
    }
}

fn calc_avg_distance_components(s: &[ScheduleEntry]) -> f32 {
    let mut last_seen = HashMap::new();
    let mut dsts = HashMap::new();

    for (i, person) in s.iter().map(|e| &e.name).enumerate() {
        let last_seen_i = last_seen.entry(person).or_insert(i);
        let dsts = dsts.entry(person).or_insert_with(Vec::new);
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

fn calc_ifneedbe_components(s: &mut Schedule, data: &[PollColumn]) -> f32 {
    let mut result = 0.0;
    for (i, person) in s.iter_mut().map(|e| &mut e.name).enumerate() {
        if let Some(Response::IfNeedBe) = data[i].responses.get(person) {
            result += 0.25;
            person.push('?');
        }
    }
    result
}

fn evaluate(mut s: Schedule, data: &[PollColumn]) -> EvaluatedSchedule {
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
