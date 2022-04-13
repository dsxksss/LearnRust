pub struct SLog {
    pub log_level: ELogLevel,
}
impl SLog {
    pub fn error(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Error) {
            println!("[Error]:\u{001b}[31m {} \u{001b}[0m", msg);
        }
    }
    pub fn warning(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Warning) {
            println!("[Warning]:\u{001b}[33m {} \u{001b}[0m", msg);
        }
    }
    pub fn info(&self, msg: &str) -> () {
        if self.log_level.get_level() >= SLog::get_level(ELogLevel::Info) {
            println!("[Info]:\u{001b}[34m {} \u{001b}[0m", msg);
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
