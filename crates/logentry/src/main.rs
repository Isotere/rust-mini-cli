use logentry::LogEntry;

fn main() {
    let line = "INFO: application started";
    match LogEntry::parse(line) {
        Some(entry) => println!("[{}] {}", entry.level, entry.message),
        None => eprintln!("не удалось разобрать строку: {line}"),
    }
}
