use std::fmt::Display;
use std::io::Error;

pub struct SLog {
    pub log_level: ELogLevel,
    pub pathname: String,
}
impl SLog {
    pub fn error(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Error) {
            println!("[Error]:\u{001b}[31m {} \u{001b}[0m", msg);
            write_file(&self.pathname, format!("[Error]: {}", String::from(msg)))
                .expect("Error write error!");
        }
    }
    pub fn warning(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Warning) {
            println!("[Warning]:\u{001b}[33m {} \u{001b}[0m", msg);
            write_file(&self.pathname, format!("[Warning]: {}", String::from(msg)))
                .expect("Warning write error!");
        }
    }
    pub fn info(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Info) {
            println!("[Info]:\u{001b}[34m {} \u{001b}[0m", msg);
            write_file(&self.pathname, format!("[Info]: {}", String::from(msg)))
                .expect("Info write error!");
        }
    }
    pub fn get_level(level: ELogLevel) -> u8 {
        match level {
            ELogLevel::Error => 0,
            ELogLevel::Warning => 1,
            ELogLevel::Info => 2,
        }
    }
}

pub enum ELogLevel {
    Info,
    Error,
    Warning,
}

impl ELogLevel {
    pub fn get_level(&self) -> u8 {
        match &self {
            ELogLevel::Error => 0,
            ELogLevel::Warning => 1,
            ELogLevel::Info => 2,
        }
    }
}

fn get_time() -> String {
    use chrono::prelude::*;
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    format!("{}", now)
}

fn write_file<T: Display>(pathname: &str, writedata: T) -> Result<(), Error> {
    use std::fs::{File, OpenOptions};
    use std::io::{prelude::*, ErrorKind};

    File::open(pathname).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(pathname).unwrap_or_else(|error| {
                panic!("Error creating file : {:?}", error);
            })
        } else {
            panic!("Error opening file : {:?}", error);
        }
    });

    let mut write_file = OpenOptions::new().append(true).open(pathname)?;
    write_file.write_fmt(format_args!("{}\t\t[{}]\n", writedata, get_time()))?;
    Ok(())
}
