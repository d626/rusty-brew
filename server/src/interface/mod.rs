use std::fs;
use std::fs::File;
use std::io;
use std::collections::HashMap;

use rocket::Rocket;
use rocket::State;
use rocket::Request;
use rocket_contrib::json::Json;

use super::controller::{Controller, ReferenceSeries};
use super::controller::sensor::Sensor;
use super::controller::output::Output;
use super::log;
use super::log::LogEntry;

type ResourceMap = HashMap<String, Controller>;

// Called by the application at startup, not part of the API
pub fn init_interface<S, O>(resources: Vec<Controller>)
where S: 'static + Sensor,
      O: 'static + Output
{
    // Make a managed map of resources (using a name as index) (using a macro?)
    // the values need to be trait objects to make it possible to have different
    // kinds of controllers (with different types of Sensor and Output)
    let resources: HashMap<String, Controller> = HashMap::new();
    rocket::ignite()
        .manage(resources)
        .mount("/", routes![
            get_list_of_logs,
            get_log,
            delete_log,
            get_current_values,
            get_list_of_resources,
            get_list_of_reference_series,
            get_reference_series,
            delete_reference_series,
            post_reference_series,
            start_controlling
        ])
        .launch();
    unimplemented!();
}

// get /logs
#[get("/logs")]
fn get_list_of_logs() -> Json<Vec<String>> {
    Json(log::get_list_of_logs())
}

// get /logs/<name>
// use query string? i.e. get /logs?<name>
// can fail if <name> does not exist
#[get("/logs/<name>")]
fn get_log(name: String) -> io::Result<File> {
    log::get_log(name)
}

// delete /logs/<name>
// can fail if <name> does not exist
#[delete("/logs/<name>")]
fn delete_log(name: String, resources: State<ResourceMap>) -> io::Result<()> {
    for (_, controller) in &(*resources) {
        if controller.get_name().is_none() || controller.get_name().unwrap() == name {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The logfile is in use, stop the process to release the logfile")
                      )
        }
    }
    // As no controller in uses the file, it can safely be deleted
    // Move this to log module?
    fs::remove_file(format!("logs/{}.log", name))
}

// get /<resource>/values
// probably not use query string?
// can fail if <resource> does not exist
// The resources are made at startup, but values does only get updated when the
// resource is in use, there are therefore no guarantee that current_values are
// up to date (but it will be the last meassured values)
#[get("/<resource>/values")]
fn get_current_values(resource: String, resources: State<ResourceMap>) -> Option<LogEntry> {
    if let Some(controller) = resources.get(&resource) {
        controller.get_last_log_entry()
    } else {
        None
    }
}

// get /resources
#[get("/resources")]
fn get_list_of_resources(resources: State<ResourceMap>) -> Json<Vec<String>> {
    // Return a list of keys in the managed map
    Json((*resources).keys().cloned().collect())
}

// get /reference_series
#[get("/reference_series")]
fn get_list_of_reference_series() -> Json<Vec<String>> {
    // Return a list of all stored ReferanceSeries
    let mut result = Vec::new();
    for file in fs::read_dir("logs").expect("Unable to read log folder") {
        result.push(file.expect("Fail while reading log folder")
                    .file_name()
                    .into_string()
                    .expect("Logname not a valid string"));
    }
    Json(result)
}

// get /reference_series/<name>
// can fail if <name> does not exist
#[get("/reference_series/<name>")]
fn get_reference_series(name: String) -> io::Result<String> {
    fs::read_to_string(name)
}

// delete /reference_series/<name>
// can fail if <name> does not exist
#[delete("/reference_series/<name>")]
fn delete_reference_series(name: String) -> io::Result<()> {
    fs::remove_file(format!("/reference_series/{}", name))
}

// post /reference_series/<name> data = <reference_series>
// can fail if <name> exists
#[post("/reference_series/<name>", data = "<reference_series>")]
fn post_reference_series(name: String, reference_series: Json<ReferenceSeries>) -> io::Result<()> {
    // If file exists, return error
    for file in fs::read_dir("reference").expect("Unable to read reference folder") {
        if file?.file_name().into_string().unwrap() == name {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The reference series already exists"));
        }
    }
    // Write json to file
    fs::write(format!("reference/{}", name), reference_series.to_string())
}

// get /start/<resource>/<profile>
// can fail if <resource> OR <profile> does not exist
// should it include the name of the beer?
// should Controller and ReferenceSeries impl some trait, or shoult this be a helper function taking input from somewhere else?
#[get("/start/<resource>/<profile>")]
fn start_controlling(resource: String, profile: String) {
    unimplemented!();
}
