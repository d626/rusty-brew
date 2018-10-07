use std::fs::File;

use super::controller::{Controller, ReferenceSeries};
use super::controller::sensor::Sensor;
use super::controller::output::Output;
use super::log::LogEntry;

// Called by the application at startup, not part of the API
pub fn init_interface(/*list of resources*/) {
    // Make a managed map of resources (using a name as index) (using a macro?)
    // the values need to be trait objects to make it possible to have different
    // kinds of controllers (with different types of Sensor and Output)
    unimplemented!();
}

// get /logs
fn get_list_of_logs() -> Vec<String> {
    unimplemented!();
}

// get /logs/<name>
// use query string? i.e. get /logs?<name>
// can fail if <name> does not exist
fn get_log(name: String) -> File {
    unimplemented!();
}

// delete /logs/<name>
// can fail if <name> does not exist
fn delete_log(name: String) {
    unimplemented!();
}

// get /<resource>/values
// probably not use query string?
// can fail if <resource> does not exist
// The resources are made at startup, but values does only get updated when the
// resource is in use, there are therefore no guarantee that current_values are
// up to date (but it will be the last meassured values)
fn get_current_values(resource: String) -> LogEntry {
    // Get last_value of Resources[resource]
    unimplemented!();
}

// get /resources
fn get_list_of_resources() -> Vec<String> {
    // Return a list of keys in the managed map
    unimplemented!();
}

// get /reference_series
fn get_list_of_reference_series() -> Vec<String> {
    // Return a list of all stored ReferanceSeries
    unimplemented!();
}

// get /reference_series/<name>
// can fail if <name> does not exist
fn get_reference_series(name: String) -> ReferenceSeries {
    unimplemented!();
}

// delete /reference_series/<name>
// can fail if <name> does not exist
fn delete_reference_series(name: String) {
    unimplemented!();
}

// post /reference_series/<name> data = <reference_series>
// can fail if <name> exists
fn post_reference_series(name: String, reference_series: ReferenceSeries) {
    unimplemented!();
}

// get /start/<resource>/<profile>
// can fail if <resource> OR <profile> does not exist
fn strart_controlling<S, O>(resource: Controller<S, O>, profile: ReferenceSeries)
    where S: Sensor, O: Output {
    unimplemented!();
}
