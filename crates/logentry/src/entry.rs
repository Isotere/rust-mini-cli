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

impl LogEntry {
    /// Разбирает строку вида `timestamp:LEVEL:message`.
    /// Нужно ровно 3 части — иначе `None`.
    /// Лишние `:` остаются внутри message (splitn ограничивает срез тремя частями).
    ///
    /// ВАЖНО
    /// Парсер ломается на реальных timestamp с :. splitn(3, ':') по разделителю :, но ISO-время само содержит ::
    /// 2026-06-07T12:00:00:INFO:msg  →  ["2026-06-07T12", "00", "00:INFO:msg"]
    /// Для хардкод-MVP ок, но это ядро лог-анализатора. Будущее: другой разделитель (\t/|), либо распознавать level/timestamp по форме, либо regex.
    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.splitn(3, ':');
        let timestamp = parts.next()?.trim().to_string();
        let level = parts.next()?.trim().to_string();
        let message = parts.next()?.trim().to_string();

        Some(Self {
            timestamp,
            level: LogLevel::from(level.as_str()),
            message,
        })
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
    fn parse_valid_line() {
        let entry = LogEntry::parse("2026-06-07:INFO:started").unwrap();
        assert_eq!(entry.timestamp, "2026-06-07");
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "started");
    }

    #[test]
    fn message_keeps_inner_colons() {
        // Двоеточия после второго остаются в message.
        let entry = LogEntry::parse("ts:WARN:user:42 logged in").unwrap();
        assert_eq!(entry.message, "user:42 logged in");
        // WARN — алиас Warning, уровень должен распознаться (не Unknown).
        assert_eq!(entry.level, LogLevel::Warning);
    }

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
    fn trims_whitespace() {
        let entry = LogEntry::parse(" ts : INFO : hello ").unwrap();
        assert_eq!(entry.timestamp, "ts");
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "hello");
    }

    #[test]
    fn too_few_parts_is_none() {
        assert_eq!(LogEntry::parse("ts:INFO"), None);
        assert_eq!(LogEntry::parse("just-text"), None);
    }
}
