use crate::entry::{LogEntry, LogLevel};

pub trait Predicate {
    fn keep(&self, entry: &LogEntry) -> bool;
}

pub struct MinLevel(pub LogLevel);
pub struct Contains(pub String);

impl Predicate for MinLevel {
    fn keep(&self, entry: &LogEntry) -> bool {
        entry.level() >= self.0
    }
}

impl Predicate for Contains {
    fn keep(&self, entry: &LogEntry) -> bool {
        entry.message().contains(&self.0)
    }
}

impl<F: Fn(&LogEntry) -> bool> Predicate for F {
    fn keep(&self, entry: &LogEntry) -> bool {
        self(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(level: LogLevel, message: &str) -> LogEntry {
        LogEntry::new("ts".to_string(), level, message.to_string())
    }

    #[test]
    fn min_level_keeps_equal_and_above() {
        let err = entry(LogLevel::Error, "boom");
        assert!(MinLevel(LogLevel::Error).keep(&err)); // равный — проходит
        assert!(MinLevel(LogLevel::Warning).keep(&err)); // выше порога
        assert!(!MinLevel(LogLevel::Fatal).keep(&err)); // ниже порога — отсев
    }

    #[test]
    fn min_level_unknown_is_lowest() {
        let unknown = entry(LogLevel::Unknown, "?");
        // Unknown ниже всех — порог Debug его отсекает.
        assert!(!MinLevel(LogLevel::Debug).keep(&unknown));
        // но порог Unknown его пропускает (равный).
        assert!(MinLevel(LogLevel::Unknown).keep(&unknown));
    }

    #[test]
    fn contains_finds_substring() {
        let e = entry(LogLevel::Info, "user 42 logged in");
        assert!(Contains("logged".to_string()).keep(&e));
        assert!(Contains("42".to_string()).keep(&e));
        assert!(!Contains("timeout".to_string()).keep(&e));
    }

    #[test]
    fn contains_is_case_sensitive() {
        let e = entry(LogLevel::Info, "Timeout");
        assert!(Contains("Timeout".to_string()).keep(&e));
        assert!(!Contains("timeout".to_string()).keep(&e));
    }

    #[test]
    fn closure_is_a_predicate() {
        // blanket impl: любой Fn(&LogEntry) -> bool тоже Predicate.
        let e = entry(LogLevel::Fatal, "x");
        let only_fatal = |e: &LogEntry| e.level() == LogLevel::Fatal;
        assert!(only_fatal.keep(&e));
        assert!(!only_fatal.keep(&entry(LogLevel::Info, "x")));
    }
}
