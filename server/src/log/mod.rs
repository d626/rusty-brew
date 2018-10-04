use std::time::SystemTime;
use std::fs;
use std::path::Path;

use chrono::prelude::*;

pub struct LogName(&str, SystemTime);

pub struct Logger(&str);

impl Logger {
    pub fn new(name: &str) -> Logger {
        let date = Local::now(); // Local date
        let name = format!("{}_{}", name, date);
        unimplemented!();
    }

    pub fn new_log(name: &str) {
        unimplemented!();
    }

    pub fn get_log(name: &str) -> &str {
        unimplemented!();
    }

    pub fn get_list_of_logs() -> Vec<LogName> {
        unimplemented!();
    }

}
