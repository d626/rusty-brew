use std::time::SystemTime;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Sender;

use super::controller::ReferenceSeries;

use chrono::prelude::*;

pub type LoggerChannel = Sender<LogEntry>;

pub struct LogEntry {
    timestamp: SystemTime,
    reference: f32,
    measured_value: f32,
    output: f32, // TODO? change type of this field to something that better
                 // represents the discrete nature of the outputs.
}

pub struct Logger {
    name: String,
    last_entry: Option<LogEntry>,
}

impl Logger {
    /// Makes a new Logger (and therefore log). Name is the name of the logfile,
    pub fn new(name: String, reference: ReferenceSeries) -> Logger {
        let date = Local::now(); // Local date
        let name = format!("{}_{}.log", name, date);
        let logger = Logger {
            name,
            last_entry: None
        };
        unimplemented!();
    }

    fn new_log(&self, reference_series: ReferenceSeries) -> std::io::Result<()> {
        let mut logfile = self.open(true)?;
        // TODO: what to do if the file exists? or other error?
        // It could return an (std::io)Error, or on succsess a ()
        logfile.write(reference_series.to_string().as_bytes())?;
        Ok(())
    }

    fn open(&self, create: bool) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .append(true)
            .create(create)
            .open(&self.name)
    }

}

pub fn get_log(name: &str) -> std::io::Result<String> {
    unimplemented!();
}

pub fn get_list_of_logs() -> Vec<String> {
    unimplemented!();
}

