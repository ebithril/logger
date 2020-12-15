use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum Severity {
    Log,
    Warning,
    Error
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Severity::Log => write!(f, "Log"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
        }
    }
}

pub fn log(severity: Severity, category: &str, message: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.log")
        .unwrap();

    let log_string = format!("[{}][{}]: {}\n", severity, category, message);

    let _result = file.write_all(log_string.as_bytes());
}
