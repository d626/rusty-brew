//! Module containig all the temperature controller logic

use std::time::Duration;
use std::string::ToString;
use std::fmt::Display;
use std::fmt;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::channel;
use std::fs;
use std::io;

pub mod sensor;
pub mod output;
pub mod mock;

pub mod ds18b20;
pub mod led;

pub mod pid;

use self::sensor::Sensor;
use self::output::Output;
use self::pid::*;
use log::{Logger, LogEntry};

/// Function for getting a list of all reference series that are stored.
pub fn get_list_of_reference_series() -> Vec<String> {
    // Return a list of all stored ReferanceSeries
    let mut result = Vec::new();
    for file in fs::read_dir("references").expect("Unable to read references folder") {
        result.push(file.expect("Fail while reading log folder")
                    .file_name()
                    .into_string()
                    .expect("Logname not a valid string"));
    }
    result
}

/// Function for getting a stored reference series.
pub fn get_reference_series(name: &String) -> io::Result<String> {
    fs::read_to_string(format!("references/{}", name))
}

/// Function for deleting a stored reference series.
pub fn delete_reference_series(name: String) -> io::Result<()> {
    fs::remove_file(format!("references/{}", name))
}

/// Function for storing a new reference series.
pub fn store_reference_series(name: String, reference_series: ReferenceSeries)
                              -> io::Result<()> {
    fs::write(format!("references/{}", name),
              serde_json::to_string(&reference_series)
                  .expect("Invalid reference series JSON"))
}

/// Struct containing a series of References.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceSeries( pub Vec<Reference> );

impl ReferenceSeries {
    pub fn new(references: Vec<Reference>) -> ReferenceSeries {
        ReferenceSeries(references)
    }
}

impl Display for ReferenceSeries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for Reference{duration, temp} in &self.0 {
            result.push_str(&duration.to_string());
            result.push_str(": ");
            result.push_str(&temp.to_string());
            result.push_str(", ");
        }
        write!(f, "{}", result)
    }
}

/// Struct representing a controller, with its own input, output and tuning.
pub struct Controller {
    logger: Arc<Mutex<Option<Logger>>>,
    sensor: Arc<Mutex<Box<'static + Sensor>>>,
    output: Arc<Mutex<Box<'static + Output>>>,
    frequency: u64,
    pid_parameters: PidParameters,
}

impl Controller {
    /// Constructor for Controller.
    /// sensor is the object used to measure the process,
    /// while output is the object used to control the process.
    /// pid_parameters is the parameters used to tune the PID controller.
    /// Frequency is the frequency the controller is running on.
    pub fn new<S, O>(sensor: S, output: O, pid_parameters: PidParameters, frequency: u64)
                     -> Controller
    where S: 'static + Sensor + Sync + Send,
          O: 'static + Output + Sync + Send,
    {
        Controller {
            pid_parameters,
            frequency,
            sensor: Arc::new(Mutex::new(Box::new(sensor))),
            output: Arc::new(Mutex::new(Box::new(output))),
            logger: Arc::new(Mutex::new(None)),
        }
    }

    /// This fuctions starts the controller, which will follow the given reference series.
    /// At the moment this function will never return an Err value. It can however panic
    /// if a logfile with the same name (same reference series and date).
    ///
    /// The function spawns three threads:
    /// - A timer thread
    /// - A reference thread
    /// - A pid thread
    ///
    /// These threads comunicate with each other using channels. The timer thread
    /// sends a message to the other two threads with a frequency given by the
    /// Controller object. The reference thread keeps track of how far we are in
    /// the process, and sends an updated reference to the pid thread when there is one.
    /// The pid thread takes its references from the reference thread, and calculates
    /// and sets a new output for each tick given by the timer thread.
    /// All these threads works from inside a fourth thread responisble for cleanup
    /// when the process is finished.
    pub fn start(&mut self, reference_name: String, reference_series: ReferenceSeries)
                 -> std::io::Result<()> {
        let logger = Logger::new(reference_name.clone());
        {
            *self.logger.lock().expect("Unable to lock logger") = Some(logger);
        }

        let logger = Arc::clone(&self.logger);
        let output = Arc::clone(&self.output);
        let sensor = Arc::clone(&self.sensor);
        let parameters = self.pid_parameters.clone();
        let period = 1000 / self.frequency;
        let period = Duration::from_millis(period);

        // println!("period: {:?}", period);
        // println!("reference series: {:?}", reference_series);

        // Make new thread to make function return immediately
        thread::spawn(move || {
            println!("Thread spawned");
            let (r_tx, r_rx) = channel();
            let (timer_tx, timer_rx) = channel();

            // Spawn thread that generates ticks for the pid
            thread::spawn(move || {
                println!("Timer spawned");
                loop {
                    thread::sleep(period);
                    match timer_tx.send(()) {
                        Ok(_) => {}, // no op
                        Err(_) => return
                    };
                }
            });

            // Spawn thread that keeps track of the reference
            thread::spawn(move || {
                println!("References spawned");
                for reference in reference_series.0 {
                    println!("new reference: {}", reference.temp);
                    r_tx.send(reference.temp).expect("r_tx failed"); // This should always work
                    thread::sleep(Duration::from_secs(reference.duration));
                }
            });

            let logger_ref = Arc::clone(&logger);
            let output_ref = Arc::clone(&output);

            // Spawn pid thread
            thread::spawn(move || {
                println!("Pid spawned");
                let mut pid = Pid::new(&parameters);
                let mut old_r = match r_rx.recv() {
                    Ok(r) => r,
                    Err(_) => return, // TODO: Should an empty reference series fail?
                };

                loop {
                    let _ = timer_rx.recv().expect("Timer thread has died prematurely");
                    let r = match r_rx.try_recv() {
                        Ok(reference) => {
                            println!("new referenc received: {}", reference);
                            old_r = reference;
                            reference
                        },
                        Err(e) => match e {
                            TryRecvError::Empty => old_r,
                            TryRecvError::Disconnected => return,
                        }
                    };

                    let y = sensor.lock().expect("Unable to lock sensor").read();
                    let u = pid.pid(y, r as f32);
                    output_ref.lock().expect("Unable to lock output").set(u);
                    {
                        let logger = &mut *logger_ref.lock().expect("Unable to lock logger");
                        logger.as_mut().expect("Unable to take logger as mut").add_entry(r as f32, y, u);
                    }
                };
            }).join().expect("Unable to join pid thread"); // The thread should not panic, unless something has gone horribly wrong

            println!("Controller finished");
            output.lock().expect("Unable to lock output").turn_off();
            *logger.lock().expect("Unable to lock logger") = None;
        });

        Ok(())
    }

    /// Function for getting the last saved log entry. Note if there are no process
    /// running there are no log entry stored.
    pub fn get_last_log_entry(&self) -> Option<LogEntry> {
        match *self.logger.lock().expect("Unable to lock logger") {
            Some(ref logger) => logger.get_last_entry(),
            None => None
        }
    }

    /// Function for getting the name of the current process/the reference series
    /// used by the current process.
    pub fn get_name_of_current_process(&self) -> Option<String> {
        match *self.logger.lock().expect("Unable to lock logger") {
            Some(ref logger) => Some(logger.get_name()),
            None => None
        }
    }
}
