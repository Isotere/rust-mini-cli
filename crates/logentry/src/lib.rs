//! LogEntry — учебный проект разбора записей лога.

/// Одна разобранная запись лога.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
}

impl LogEntry {
    /// Разбирает строку вида `LEVEL: message`.
    pub fn parse(line: &str) -> Option<LogEntry> {
        let (level, message) = line.split_once(':')?;
        Some(LogEntry {
            level: level.trim().to_string(),
            message: message.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let entry = LogEntry::parse("INFO: started").unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "started");
    }

    #[test]
    fn parse_invalid_line() {
        assert_eq!(LogEntry::parse("no separator"), None);
    }
}
