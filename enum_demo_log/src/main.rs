enum LogV {
    Info,
    Error,
    Warning,
}

impl LogV {
    fn print_log(&self, msg: &String) -> () {
        match &self {
            LogV::Error => println!("\u{001b}[31m {} \u{001b}[0m", { msg }),
            LogV::Warning => println!("\u{001b}[33m {} \u{001b}[0m", { msg }),
            LogV::Info => println!("\u{001b}[34m {} \u{001b}[0m", { msg }),
        }
    }
}

fn main() {
    LogV::Error.print_log(&String::from("i am info msg"));
    LogV::Warning.print_log(&String::from("i am info msg"));
    LogV::Info.print_log(&String::from("i am info msg"));
}
