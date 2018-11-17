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

#[derive(Serialize, Deserialize)]
pub struct Log {
    reference: String,
    entries: Vec<LogEntry>
}

impl Log {
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

#[derive(Clone, Serialize, Deserialize)]
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

// TODO: Replace old logger (and finish implement this)
pub struct Logger {
    last_entry: Option<LogEntry>,
    name: String,
}

impl Logger {
    pub fn new(reference: String) -> Logger {
        // TODO: Make file
        let log = Log::new(&reference);

        let date = Local::now();
        let date = date.format("%Y-%m-%d");

        let name = format!("logs/{}-{}", reference, date);

        fs::write(&name, serde_json::to_string(&log).expect("Unable to make JSON"))
            .expect(&format!("Unable asdfasdfas to write logfile {}", name));

        Logger {
            name,
            last_entry: None,
        }
    }

    pub fn add_entry(&mut self, reference: f32, input: f32, output: f32) {
        let tmp_str = fs::read_to_string(&self.name)
            .expect(&format!("Unable to open lkjhlkjhlhl logfile: {}", self.name));
        let mut log: Log = serde_json::from_str(
            &fs::read_to_string(&self.name)
                .expect(&format!("Unable to open lkjhlkjhlhl logfile: {}", self.name))
        ).expect(&format!("Invalid JSON in logfile: {}", tmp_str)); // We wrote this file, and it should be valid JSON

        log.add_entry(LogEntry::new(reference, input, output));

        fs::write(&self.name, serde_json::to_string(&log).unwrap())
            .expect(&format!("Unable to write logfile {}", self.name));
    }

    pub fn get_last_entry(&self) -> Option<LogEntry> {
        self.last_entry.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
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
