use crate::entry::{LogEntry, LogLevel};

pub fn by_level(entry: &LogEntry, level: LogLevel) -> bool {
    entry.level() == level
}

pub fn message_contains(entry: &LogEntry, needle: &str) -> bool {
    entry.message().contains(needle)
}

pub fn min_level(entry: &LogEntry, threshold: LogLevel) -> bool {
    entry.level() >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(level: LogLevel, message: &str) -> LogEntry {
        LogEntry::new("ts".to_string(), level, message.to_string())
    }

    #[test]
    fn by_level_matches_exact() {
        let e = entry(LogLevel::Warning, "msg");
        assert!(by_level(&e, LogLevel::Warning));
        assert!(!by_level(&e, LogLevel::Error));
    }

    #[test]
    fn message_contains_finds_substring() {
        let e = entry(LogLevel::Info, "user 42 logged in");
        assert!(message_contains(&e, "logged"));
        assert!(message_contains(&e, "42"));
        assert!(!message_contains(&e, "timeout"));
    }

    #[test]
    fn message_contains_is_case_sensitive() {
        let e = entry(LogLevel::Info, "Timeout");
        assert!(message_contains(&e, "Timeout"));
        assert!(!message_contains(&e, "timeout"));
    }

    #[test]
    fn min_level_includes_equal_and_above() {
        let err = entry(LogLevel::Error, "boom");
        assert!(min_level(&err, LogLevel::Error)); // равный — проходит
        assert!(min_level(&err, LogLevel::Warning)); // выше порога
        assert!(!min_level(&err, LogLevel::Fatal)); // ниже порога — отсев
    }

    #[test]
    fn min_level_unknown_is_lowest() {
        let unknown = entry(LogLevel::Unknown, "?");
        // Unknown ниже всех — порог Debug его отсекает.
        assert!(!min_level(&unknown, LogLevel::Debug));
        // но порог Unknown его пропускает (равный).
        assert!(min_level(&unknown, LogLevel::Unknown));
    }
}
