use std::io::{BufRead, Write};

use crate::{aggregator::Summary, entry::LogLevel, filter, parser};

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

    for line in reader.lines() {
        let line = line?; // io ошибка пробрасывается через ?
        let entry = match parser::parse(&line) {
            Ok(e) => e,
            Err(_) => continue, // пропускаем невалидные
        };

        if let Some(min) = opts.min_level
            && !filter::min_level(&entry, min)
        {
            continue;
        }

        if let Some(needle) = &opts.contains
            && !filter::message_contains(&entry, needle)
        {
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
