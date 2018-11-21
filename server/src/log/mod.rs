//! Module containig all functionality needed for logging of process variables.

use std::time::SystemTime;
use std::fs;
use std::fs::File;
use std::io;

use chrono::prelude::*;

/// Function for getting a logfile.
pub fn get_log(name: String) -> io::Result<File> {
    File::open(format!("logs/{}", name))
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

/// Structure representing a log. It has a field storing the name of the
/// reference series used, and a vector of LogEntrys.
#[derive(Serialize, Deserialize)]
pub struct Log {
    reference: String,
    entries: Vec<LogEntry>
}

impl Log {
    /// Makes a new log. reference is the name of the reference series used.
    pub fn new(reference: &String) -> Log {
        Log {
            reference: reference.clone(),
            entries: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }
}

/// A single entry in a Log. Stores a timestamp with millisecond precision
/// (as number of milliseconds since UNIX_EPOCH), the reference value at that
/// point in time, the meassured input and the calculated output. Note that due
/// to the possibly descreet nature of the output the actual output set might differ.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Milliseconds since UNIX_EPOCH
    timestamp: u64,
    reference: f32,
    input: f32,
    output: f32,
}

impl LogEntry {
    pub fn new(reference: f32, input: f32, output: f32) -> LogEntry {
        let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp = since_epoch.as_secs() * 1000 + since_epoch.subsec_millis() as u64;
        LogEntry {
            reference,
            input,
            output,
            timestamp,
        }
    }
}

/// Struct providing functionality to make a log.
#[derive(Debug)]
pub struct Logger {
    last_entry: Option<LogEntry>,
    name: String,
}

impl Logger {
    // TODO: Make not panic if logfile already exists

    /// Constructor for Logger. This function makes a new log with the name %r-%Y-%m-%d,
    /// where %r is the name of the reference series used, %Y-%m-%d is the date on ISO 8601
    /// format. Note that is the fiel already exists this function panics.
    pub fn new(reference: String) -> Logger {
        // TODO: Make file
        let log = Log::new(&reference);

        let date = Local::now();
        let date = date.format("%Y-%m-%d");

        let name = format!("logs/{}-{}", reference, date);

        fs::write(&name, serde_json::to_string(&log).expect("Unable to make JSON"))
            .expect(&format!("Unable to write logfile {}", name));

        Logger {
            name,
            last_entry: None,
        }
    }

    pub fn add_entry(&mut self, reference: f32, input: f32, output: f32) {
        let tmp_str = fs::read_to_string(&self.name)
            .expect(&format!("Unable to open logfile: {}", self.name));
        let mut log: Log = serde_json::from_str(
            &fs::read_to_string(&self.name)
                .expect(&format!("Unable to open logfile: {}", self.name))
        ).expect(&format!("Invalid JSON in logfile: {}", tmp_str)); // We wrote this file, and it should be valid JSON
        let entry = LogEntry::new(reference, input, output);

        log.add_entry(entry.clone());

        fs::write(&self.name, serde_json::to_string(&log).unwrap())
            .expect(&format!("Unable to write logfile {}", self.name));
        self.last_entry = Some(entry);
    }

    pub fn get_last_entry(&self) -> Option<LogEntry> {
        self.last_entry.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
