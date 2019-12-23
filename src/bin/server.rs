#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::Data;
use rocket::Response;
use rocket_contrib::json::Json;
use rocket_cors::CorsOptions;
use rand::Rng; 
use rand::distributions::Alphanumeric;
use std::error::Error;
use simple_error::SimpleError;
use serde::Serialize;

use framaschedule::scheduling;
use framaschedule::framadate;
use framaschedule::scheduling::{BestSchedules, SchedulingOptions, EvaluatedSchedule, ScheduleEntry};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World!"
}

#[derive(Serialize)]
struct ScheduleEntryResult {
    time: String,
    name: String,
    ifneedbe: bool
}

#[derive(Serialize)]
struct ScheduleResult {
    cost: f32,
    name_counts: Vec<(String, usize)>,
    entries: Vec<ScheduleEntryResult>,
}

fn convert_schedule(mut old: EvaluatedSchedule) -> ScheduleResult {
    ScheduleResult {
        cost: old.cost,
        name_counts: old.name_counts
            .iter_mut().map(|&mut (name, size)| (name.to_string(), size))
            .collect::<Vec<(String, usize)>>(),
        entries: old.entries
            .iter_mut().map(|&mut ScheduleEntry{time, name, ifneedbe}| ScheduleEntryResult{
                time: time.to_string(), name: name.to_string(), ifneedbe
            })
            .collect::<Vec<ScheduleEntryResult>>()
    }
}

#[post("/framadate", format = "text/csv", data = "<data>")]
fn schedule(data: Data) -> Result<Json<ScheduleResult>, Box<dyn Error>> {
    // Buffer file
    let filename = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .collect::<String>();
    let filename = format!("/tmp/schedule{}.csv", &filename);
    data.stream_to_file(&filename);

    // Schedule
    let schedule_data = framadate::read_data(&filename)?;
    std::fs::remove_file(&filename);
    let result = scheduling::compute_all_schedules(&schedule_data, &SchedulingOptions::default());

    // TODO: handle invalid schedules better (rocket-wise)
    match result {
        BestSchedules::Two(r1, _) | BestSchedules::One(r1) => {
            Ok(Json(convert_schedule(r1)))
        },
        _ => Err(Box::from(SimpleError::new("No valid schedule found!")))
    }
}

#[options("/framadate")]
fn options_handler<'a>() -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "*")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "*")
        .finalize()
}


fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsOptions::default().to_cors()?;
    rocket::ignite()
        .mount("/scheduling/api", routes![hello, schedule])
        .attach(cors)
        .launch();

    Ok(())
}