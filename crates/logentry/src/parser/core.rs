use std::sync::OnceLock;

use regex::Regex;

use crate::{
    entry::{LogEntry, LogLevel},
    parser::ParseError,
};

fn line_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
          Regex::new(
              r"(?i)^\s*(?P<ts>.+?)\s*[:\s]\s*(?P<level>DEBUG|INFO|WARNING|WARN|ERROR|ERR|FATAL)\s*[:\s]\s*(?P<msg>.+?)\s*$"
          ).expect("hardcoded regex must compile") // expect ок: паттерн константный, провал = баг разработчика
      })
}

pub fn parse(line: &str) -> Result<LogEntry, ParseError> {
    if line.trim().is_empty() {
        return Err(ParseError::Empty);
    }

    let caps = line_re()
        .captures(line)
        .ok_or_else(|| ParseError::BadFormat(line.to_string()))?;

    let timestamp = caps["ts"].trim().to_string();
    let level = LogLevel::from(caps["level"].trim());
    let message = caps["msg"].trim().to_string();

    Ok(LogEntry::new(timestamp, level, message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let entry = parse("2026-06-07 12:00:00:INFO:started").unwrap();
        assert_eq!(entry.timestamp(), "2026-06-07 12:00:00");
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
        assert_eq!(
            parse("ts:INFO"),
            Err(ParseError::BadFormat("ts:INFO".to_string()))
        );
        assert_eq!(
            parse("just-text"),
            Err(ParseError::BadFormat("just-text".to_string()))
        );
    }

    #[test]
    fn empty_or_blank_is_empty_error() {
        // Empty branch: nothing to parse -> distinct error, not BadFormat.
        assert_eq!(parse(""), Err(ParseError::Empty));
        assert_eq!(parse("   "), Err(ParseError::Empty));
        assert_eq!(parse(" \t \n "), Err(ParseError::Empty));
    }

    #[test]
    fn level_is_case_insensitive() {
        // (?i) flag + space separator [:\s]: level recognized regardless of case.
        assert_eq!(parse("ts info x").unwrap().level(), LogLevel::Info);
        assert_eq!(parse("ts WaRn x").unwrap().level(), LogLevel::Warning);
        assert_eq!(parse("ts error x").unwrap().level(), LogLevel::Error);
        assert_eq!(parse("ts FaTaL x").unwrap().level(), LogLevel::Fatal);
    }
}
