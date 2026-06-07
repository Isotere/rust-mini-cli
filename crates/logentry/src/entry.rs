/// Одна разобранная запись лога.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

impl LogEntry {
    /// Разбирает строку вида `timestamp:LEVEL:message`.
    /// Нужно ровно 3 части — иначе `None`.
    /// Лишние `:` остаются внутри message (splitn ограничивает срез тремя частями).
    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.splitn(3, ':');
        let timestamp = parts.next()?.trim().to_string();
        let level = parts.next()?.trim().to_string();
        let message = parts.next()?.trim().to_string();

        Some(Self {
            timestamp,
            level,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let entry = LogEntry::parse("2026-06-07:INFO:started").unwrap();
        assert_eq!(entry.timestamp, "2026-06-07");
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "started");
    }

    #[test]
    fn message_keeps_inner_colons() {
        // Двоеточия после второго остаются в message.
        let entry = LogEntry::parse("ts:WARN:user:42 logged in").unwrap();
        assert_eq!(entry.message, "user:42 logged in");
    }

    #[test]
    fn trims_whitespace() {
        let entry = LogEntry::parse(" ts : INFO : hello ").unwrap();
        assert_eq!(entry.timestamp, "ts");
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "hello");
    }

    #[test]
    fn too_few_parts_is_none() {
        assert_eq!(LogEntry::parse("ts:INFO"), None);
        assert_eq!(LogEntry::parse("just-text"), None);
    }
}
