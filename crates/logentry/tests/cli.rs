//! Integration tests: drive the pipeline as an EXTERNAL consumer of the
//! `logentry` library — only the public API (`app::run`, `app::Options`,
//! `entry::LogLevel`). Each file in `tests/` is its own crate, so anything
//! used here must be `pub`. The DI design of `run` (generic over BufRead/Write)
//! is what makes this black-box test possible without real stdin/stdout.

use logentry::app::{Options, run};
use logentry::entry::LogLevel;

#[test]
fn end_to_end_parse_filter_summarize() {
    let input = "\
2026-06-07 12:00:00:INFO:started
2026-06-07 12:00:01:ERROR:db timeout
garbage line without level
2026-06-07 12:00:02:WARN:careful
";
    let opts = Options {
        min_level: Some(LogLevel::Warning),
        ..Default::default()
    };
    let summary = run(input.as_bytes(), &mut Vec::new(), &opts).unwrap();

    // ERROR + WARN проходят; INFO < Warning отсеян; мусор пропущен.
    assert_eq!(summary.total(), 2);
    assert_eq!(summary.total_by_level(LogLevel::Error), 1);
    assert_eq!(summary.total_by_level(LogLevel::Warning), 1);
    assert_eq!(summary.total_by_level(LogLevel::Info), 0);
}

#[test]
fn show_outputs_only_kept_lines() {
    let input = "ts:ERROR:boom\nts:INFO:noise\n";
    let opts = Options {
        min_level: Some(LogLevel::Error),
        show: true,
        ..Default::default()
    };
    let mut out = Vec::new();
    run(input.as_bytes(), &mut out, &opts).unwrap();

    let printed = String::from_utf8(out).unwrap();
    assert!(printed.contains("boom"));
    assert!(!printed.contains("noise")); // INFO отфильтрован
}

#[test]
fn default_options_keep_everything_valid() {
    // Без фильтров (Options::default) — считаем все валидные строки, мусор пропускаем.
    let input = "ts:INFO:a\nts:ERROR:b\nbad\n";
    let summary = run(input.as_bytes(), &mut Vec::new(), &Options::default()).unwrap();
    assert_eq!(summary.total(), 2);
}

#[test]
fn contains_filters_by_substring() {
    let input = "ts:ERROR:db timeout\nts:ERROR:ok\n";
    let opts = Options {
        contains: Some("timeout".to_string()),
        ..Default::default()
    };
    let summary = run(input.as_bytes(), &mut Vec::new(), &opts).unwrap();
    assert_eq!(summary.total(), 1);
}
