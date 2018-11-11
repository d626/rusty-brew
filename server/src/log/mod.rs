use std::time::SystemTime;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::mpsc::Sender;

use super::controller::ReferenceSeries;

use chrono::prelude::*;

use rocket::Request;
use rocket::response;
use rocket::response::Responder;

pub type LoggerChannel = Sender<LogEntry>;

#[derive(Clone)]
pub struct LogEntry {
    timestamp: SystemTime,
    reference: f32,
    measured_value: f32,
    output: f32, // TODO? change type of this field to something that better
                 // represents the discrete nature of the outputs.
}

impl<'r> Responder<'r> for LogEntry {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        // is it possible to implement Responder for SystemTime? Can i convert SystemTime to something that implements Responder?
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct Logger {
    name: String,
    last_entry: Option<LogEntry>,
}

impl Logger {
    /// Makes a new Logger (and therefore log). Name is the name of the logfile,
    /// Returns an Err if the file already exists (or is otherwise impossible to create)
    pub fn new(name: String, reference: ReferenceSeries) -> io::Result<Logger> {
        let date = Local::now(); // Local date
        let name = format!("{}_{}.log", name, date.format("%Y-%m-%d"));
        let logger = Logger {
            name,
            last_entry: None
        };
        logger.new_log(reference)?;
        Ok(logger)
    }

    // Should this function be allowed to fail
    // TODO: Change to store JSON
    pub fn add_entry(&mut self, entry: LogEntry) -> std::io::Result<()> {
        let mut file = self.open(false)?;
        file.write(entry.to_string().as_bytes())?;
        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_last_entry(&self) -> Option<LogEntry> {
        self.last_entry.clone()
    }

    fn new_log(&self, reference_series: ReferenceSeries) -> io::Result<()> {
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

impl ToString for LogEntry {
    fn to_string(&self) -> String {
        format!("{},{},{},{}\n",
                self.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                self.reference, self.measured_value,
                self.output)
        // Shoud I use absolute or relative timestamps? (how many batches can be brewed in a day?)
    }
}

pub fn get_log(name: String) -> io::Result<File> {
    File::open(name)
}

pub fn get_list_of_logs() -> Vec<String> {
    let mut result = Vec::new();
    for file in fs::read_dir("logs").expect("Unable to read log folder") {
        result.push(file.expect("Fail while reading log folder")
                    .file_name()
                    .into_string()
                    .expect("Logname not a valid string"));
    }
    result
}

// TODO: Replace old logger (and finish implement this)
pub struct NewLogger {
    last_entry: Option<LogEntry>,
    name: String,
}

impl NewLogger {
    pub fn new(name: String) -> NewLogger {
        // Make file
        NewLogger {
            name,
            last_entry: None,
        }
    }

    pub fn add_entry(&mut self, reference: f32, input: f32, output: f32) {
        unimplemented!();
    }

    pub fn get_last_entry(&self) -> Option<LogEntry> {
        self.last_entry.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
