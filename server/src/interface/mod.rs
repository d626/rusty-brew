// use std::fs;
use std::fs::File;
use std::io;
use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::Json;

use controller;
use controller::{Controller, ReferenceSeries};
use log;
use log::LogEntry;

type ResourceMap = HashMap<String, Mutex<Controller>>;

// TODO: Make custom result type, to give better error messages
// TODO: Move JSON encoding into respective modules, don't pass Strings around unnecessary

// Called by the application at startup, not part of the API

/// Initialises the web server.
/// Takes a list of controllers that will be exposed on the internet.
/// Note that this function does not return, unless there were an error starting
/// the server.
pub fn init_interface(resources: ResourceMap) 
{
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
}

/// Returns a list of all available logs.
/// Route: GET /logs
/// Returns a list of names of available logs encoded in JSON.
/// ```no_run
/// ["log1", "log2", "log3"]
/// ```
/// A specific log can be retrieved by sending a GET request to /logs/<name>
#[get("/logs")]
fn get_list_of_logs() -> Json<Vec<String>> {
    Json(log::get_list_of_logs())
}

// use query string? i.e. get /logs?<name>
// can fail if <name> does not exist

/// Returns the log <name>
/// Route: GET /logs/<name>
/// Returns a logfile.
/// Fails if the log specified doesn't exist.
/// The logfile contains a JSON encoded Log, on the following format:
/// {
///   "reference": String, // The name of the reference series used
///   "entries": [
///     {
///       "timestamp": Integer, // # milliseconds sice UNIX_EPOCH
///       "reference": Floatind point number, // Reference at given point of time
///       "input": Floating point number, // Meassured value at given point of time
///       "output": Floating point number // Output of the controller at given point of time
///     },
///     ... // The rest of the entries are skipped
///   ]
/// }
#[get("/logs/<name>")]
fn get_log(name: String) -> io::Result<File> {
    log::get_log(name)
}

// can fail if <name> does not exist

/// Deletes the specified logfile
/// Route: DELETE /logs/<name>
/// Fails if the logfile is in use by a currently running process.
#[delete("/logs/<name>")]
fn delete_log(name: String, resources: State<ResourceMap>) -> io::Result<()> {
    println!("Deleting log {}", name);
    for (_, controller) in &(*resources) {
        if !controller.lock().unwrap().get_name_of_current_process().is_none()
           && controller.lock().unwrap().get_name_of_current_process().unwrap() == name
        {
            println!("Log in use");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The logfile is in use, stop the process to release the logfile")
                      )
        }
    }
    // As no controller in uses the file, it can safely be deleted
    // Move this to log module?
    println!("Removing file");
    log::delete_log(name)
}

// probably not use query string?
// can fail if <resource> does not exist
// The resources are made at startup, but values does only get updated when the
// resource is in use, there are therefore no guarantee that current_values are
// up to date (but it will be the last meassured values)
// The rank is needed due to conflict with GET /reference_series/<name>

/// Returns the current state of the given controller
/// Route: GET /<resource>/values
/// Note that you only get the last logged values, new values are not produced
/// on request.
/// Responds with a 404 if the given controller doesn't exist, or isn't in use.
/// The names of all resources can be found using GET /resources.
#[get("/<resource>/values", rank = 2)]
fn get_current_values(resource: String, resources: State<ResourceMap>)
                      -> Option<Json<LogEntry>>
{
    println!("Getting current values for: {}", resource);
    if let Some(controller) = resources.get(&resource) {
        println!("Resource exists");
        println!("Last entry: {:?}", controller.lock().unwrap().get_last_log_entry());
        if let Some(entry) = controller.lock().unwrap().get_last_log_entry() {
            Some(Json(entry))
        } else {
            None
        }
    } else {
        None
    }
}

/// Returns a list of the name of all controllers.
/// Route: GET /resources
/// The controllers are made at compile time, and each got its own Sensor and
/// Output. The names can be used in any request taking <resource>.
#[get("/resources")]
fn get_list_of_resources(resources: State<ResourceMap>) -> Json<Vec<String>> {
    // Return a list of keys in the managed map
    Json((*resources).keys().cloned().collect())
}

/// Returns a list of saved reference series.
/// Route: GET /reference_series
/// Returns a JSON encoded list of names of saved reference series.
#[get("/reference_series")]
fn get_list_of_reference_series() -> Json<Vec<String>> {
    Json(controller::get_list_of_reference_series())
}

// can fail if <name> does not exist
// TODO: return File?

/// Returns the given reference series.
/// Route: GET /reference_series/<name>
/// Returns the JSON encoded reference series given by <name>.
/// A list of available reference series can be found by sending a GET request
/// to /reference_series. Fails if the given reference series doesn't exist, or
/// other filesystem error.
#[get("/reference_series/<name>", rank = 1)]
fn get_reference_series(name: String) -> io::Result<String> {
    controller::get_reference_series(&name)
}

// delete /reference_series/<name>
// can fail if <name> does not exist

/// Deletes the given reference series.
/// Route: DELETE /reference_series/<name>
/// Fails if the reference series doesn't exist or other filesystem error.
#[delete("/reference_series/<name>")]
fn delete_reference_series(name: String) -> io::Result<()> {
    controller::delete_reference_series(name)
}

// can fail if <name> exists

/// Save a new reference series
/// Route: POST /reference_series/<name>
/// Fails if a reference series with the same name already exists.
#[post("/reference_series/<name>", data = "<reference_series>")]
fn post_reference_series(name: String, reference_series: Json<ReferenceSeries>)
                         -> io::Result<()>
{
    // If file exists, return error
    for file in controller::get_list_of_reference_series() {
        if file == name {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists,
                                      "The reference series already exists"));
        }
    }
    // TODO: Check that received JSON is valid
    // println!("recv'd reference JSON:{:?}", reference_series.into_inner());
    // Write json to file
    controller::store_reference_series(name, reference_series.into_inner())
}

// get /start/<resource>/<profile>
// can fail if <resource> OR <profile> does not exist
// should it include the name of the beer?
// should Controller and ReferenceSeries impl some trait, or shoult this be a helper function taking input from somewhere else?

/// Start the given resource, with the given reference profile.
/// Route: GET /start/<resource>/<profile>
/// <resource> is the name of the controller to be used (i.e. one of the names from
/// GET /resources). <profile> is the name of a reference profile stored using
/// POST /references<name>. To get a list of stored refence profiles use GET /reference_series.
#[get("/start/<resource>/<profile>")]
fn start_controlling(resource: String, profile: String, resource_map: State<ResourceMap>) -> Option<()> {
    // TODO: replace file operations with calls to reference module
    println!("Starting controlling");
    let reference_series: ReferenceSeries = serde_json::from_str(
        &controller::get_reference_series(&profile).expect("Unable to read stored profile")
    ).unwrap();

    let controller = resource_map.get(&resource)?;
    controller.lock().unwrap().start(profile, reference_series).unwrap(); // TODO: find a way to return this error, rather than panic

    Some(())
}
