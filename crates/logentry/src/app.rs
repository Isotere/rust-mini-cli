use std::io::{BufRead, Write};

use crate::{
    aggregator::Summary,
    entry::LogLevel,
    filter::{self, Predicate},
    parser,
};

pub struct Options {
    pub min_level: Option<LogLevel>,
    pub contains: Option<String>,
    pub show: bool, // печатать ли каждую прошедшую строку
}

/// Читает строки из reader, парсит, фильтрует, агрегирует.
/// Прошедшие строки (если show) пишет в out. Возвращает Summary.
/// Битые/пустые строки молча пропускаются (MVP-политика).
pub fn run<R: BufRead, W: Write>(
    reader: R,
    mut out: W,
    opts: &Options,
) -> std::io::Result<Summary> {
    let mut summary = Summary::default();

    let mut predicates: Vec<Box<dyn Predicate>> = Vec::new();
    if let Some(min) = opts.min_level {
        predicates.push(Box::new(filter::MinLevel(min)));
    }

    if let Some(needle) = &opts.contains {
        predicates.push(Box::new(filter::Contains(needle.clone())));
    }

    for line in reader.lines() {
        let line = line?; // io ошибка пробрасывается через ?
        let entry = match parser::parse(&line) {
            Ok(e) => e,
            Err(_) => continue, // пропускаем невалидные
        };

        if !predicates.iter().all(|p| p.keep(&entry)) {
            continue;
        }

        if opts.show {
            writeln!(
                out,
                "{} {} {}",
                entry.timestamp(),
                entry.level(),
                entry.message()
            )?;
        }

        summary.count(&entry);
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_filters_by_min_level_and_skips_garbage() {
        let input = "\
2026-06-07 12:00:00:INFO:started
2026-06-07 12:00:01:ERROR:boom
bad line without level
2026-06-07 12:00:02:WARN:careful
";
        let opts = Options {
            min_level: Some(LogLevel::Warning),
            contains: None,
            show: false,
        };
        let mut out = Vec::new();
        let summary = run(input.as_bytes(), &mut out, &opts).unwrap();

        assert_eq!(summary.total(), 2); // ERROR+WARN проходят; INFO < Warning отсеян; мусор пропущен
        assert_eq!(summary.total_by_level(LogLevel::Error), 1);
        assert_eq!(summary.total_by_level(LogLevel::Warning), 1);
        assert_eq!(summary.total_by_level(LogLevel::Info), 0);
    }

    #[test]
    fn run_composes_min_level_and_contains() {
        // Два предиката вместе (.all()): держим строку, только если ОБА проходят.
        let input = "ts:ERROR:db timeout\nts:ERROR:ok\nts:INFO:db timeout\n";
        let opts = Options {
            min_level: Some(LogLevel::Error),
            contains: Some("timeout".to_string()),
            show: false,
        };
        let summary = run(input.as_bytes(), &mut Vec::new(), &opts).unwrap();

        // только ERROR + "timeout"; ERROR без timeout и INFO+timeout отсеяны.
        assert_eq!(summary.total(), 1);
        assert_eq!(summary.total_by_level(LogLevel::Error), 1);
    }

    #[test]
    fn run_show_writes_kept_lines() {
        let input = "ts:ERROR:boom\nts:INFO:noise\n";
        let opts = Options {
            min_level: Some(LogLevel::Error),
            contains: None,
            show: true,
        };
        let mut out = Vec::new();
        run(input.as_bytes(), &mut out, &opts).unwrap();
        let printed = String::from_utf8(out).unwrap();
        assert!(printed.contains("boom"));
        assert!(!printed.contains("noise")); // INFO отфильтрован
    }
}
