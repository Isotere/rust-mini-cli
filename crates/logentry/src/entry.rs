#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
    Unknown,
}

impl From<&str> for LogLevel {
    fn from(val: &str) -> Self {
        match val.trim().to_uppercase().as_str() {
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" | "WARNING" => LogLevel::Warning,
            "ERR" | "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => LogLevel::Unknown,
        }
    }
}

/// Одна разобранная запись лога.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    timestamp: String,
    level: LogLevel,
    message: String,
}

impl LogLevel {
    fn severity(self) -> u8 {
        match self {
            LogLevel::Unknown => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
        }
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.severity().cmp(&other.severity())
    }
}

impl LogEntry {
    pub fn new(timestamp: String, level: LogLevel, message: String) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_canonical_names() {
        assert_eq!(LogLevel::from("DEBUG"), LogLevel::Debug);
        assert_eq!(LogLevel::from("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from("WARNING"), LogLevel::Warning);
        assert_eq!(LogLevel::from("ERROR"), LogLevel::Error);
        assert_eq!(LogLevel::from("FATAL"), LogLevel::Fatal);
    }

    #[test]
    fn log_level_aliases() {
        // Реальные логи пишут сокращённо.
        assert_eq!(LogLevel::from("WARN"), LogLevel::Warning);
        assert_eq!(LogLevel::from("ERR"), LogLevel::Error);
    }

    #[test]
    fn log_level_case_insensitive() {
        assert_eq!(LogLevel::from("info"), LogLevel::Info);
        assert_eq!(LogLevel::from("Error"), LogLevel::Error);
        assert_eq!(LogLevel::from("wArN"), LogLevel::Warning);
    }

    #[test]
    fn log_level_trims_whitespace() {
        assert_eq!(LogLevel::from("  info  "), LogLevel::Info);
    }

    #[test]
    fn log_level_unrecognized_is_unknown() {
        assert_eq!(LogLevel::from("xyz"), LogLevel::Unknown);
        assert_eq!(LogLevel::from(""), LogLevel::Unknown);
    }

    #[test]
    fn log_level_ordered_by_severity() {
        // Порядок severity, не порядок объявления.
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Fatal);
    }

    #[test]
    fn log_level_unknown_is_minimum() {
        // Unknown — ниже всех известных уровней.
        assert!(LogLevel::Unknown < LogLevel::Debug);
        assert!(LogLevel::Unknown < LogLevel::Fatal);
    }

    #[test]
    fn log_level_fatal_is_maximum() {
        let mut levels = [
            LogLevel::Error,
            LogLevel::Debug,
            LogLevel::Fatal,
            LogLevel::Info,
        ];
        levels.sort();
        assert_eq!(levels.last(), Some(&LogLevel::Fatal));
        assert_eq!(levels.first(), Some(&LogLevel::Debug));
    }
}
