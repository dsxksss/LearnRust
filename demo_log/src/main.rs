use demo_log::{ELogLevel, SLog};
fn main() {
    let log = SLog {
        log_level: ELogLevel::Warning,
    };
    log.error("I am a Error msg");
    log.warning(&String::from("I am a Error msg"));
    log.info(&String::from("I am a Error msg"));
}
