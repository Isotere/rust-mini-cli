use crate::entry::{LogEntry, LogLevel};

/// Разбирает строку вида `timestamp:LEVEL:message`.
/// Нужно ровно 3 части — иначе `None`.
/// Лишние `:` остаются внутри message (splitn ограничивает срез тремя частями).
///
/// ВАЖНО
/// Парсер ломается на реальных timestamp с :. splitn(3, ':') по разделителю :, но ISO-время само содержит ::
/// 2026-06-07T12:00:00:INFO:msg  →  ["2026-06-07T12", "00", "00:INFO:msg"]
/// Для хардкод-MVP ок, но это ядро лог-анализатора. Будущее: другой разделитель (\t/|), либо распознавать level/timestamp по форме, либо regex.
pub fn parse(line: &str) -> Option<LogEntry> {
    let mut parts = line.splitn(3, ':');
    let timestamp = parts.next()?.trim().to_string();
    let level = parts.next()?.trim().to_string();
    let message = parts.next()?.trim().to_string();

    Some(LogEntry::new(
        timestamp,
        LogLevel::from(level.as_str()),
        message,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let entry = parse("2026-06-07:INFO:started").unwrap();
        assert_eq!(entry.timestamp(), "2026-06-07");
        assert_eq!(entry.level(), LogLevel::Info);
        assert_eq!(entry.message(), "started");
    }

    #[test]
    fn message_keeps_inner_colons() {
        // Двоеточия после второго остаются в message.
        let entry = parse("ts:WARN:user:42 logged in").unwrap();
        assert_eq!(entry.message(), "user:42 logged in");
        // WARN — алиас Warning, уровень должен распознаться (не Unknown).
        assert_eq!(entry.level(), LogLevel::Warning);
    }

    #[test]
    fn trims_whitespace() {
        let entry = parse(" ts : INFO : hello ").unwrap();
        assert_eq!(entry.timestamp(), "ts");
        assert_eq!(entry.level(), LogLevel::Info);
        assert_eq!(entry.message(), "hello");
    }

    #[test]
    fn too_few_parts_is_none() {
        assert_eq!(parse("ts:INFO"), None);
        assert_eq!(parse("just-text"), None);
    }
}
