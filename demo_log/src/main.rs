use demo_log::{ELogLevel, SLog};
fn main() {
    let log = SLog {
        pathname: String::from("rustDemo.log"),
        log_level: ELogLevel::Info,
    };
    log.error("I am a Error msg");
    log.warning(&String::from("I am a Warning msg"));
    log.info(&String::from("I am a Info msg"));
}
