use std::collections::BTreeMap;

use crate::entry::{LogEntry, LogLevel};

#[derive(Debug, Default)]
pub struct Summary {
    total: usize,
    by_level: BTreeMap<LogLevel, usize>,
}

impl Summary {
    pub fn total(&self) -> usize {
        self.total
    }

    pub fn total_by_level(&self, level: LogLevel) -> usize {
        self.by_level.get(&level).copied().unwrap_or(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = (LogLevel, usize)> + '_ {
        self.by_level.iter().map(|(&lvl, &cnt)| (lvl, cnt))
    }
}

impl Summary {
    pub fn count(&mut self, entry: &LogEntry) {
        let cnt = self.by_level.entry(entry.level()).or_insert(0);
        *cnt += 1;

        self.total += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_count_check() {
        let mut smry = Summary::default();

        // Empty
        assert_eq!(smry.total, 0);
        assert_eq!(smry.by_level.len(), 0);

        // Add One new
        let entry = LogEntry::new(
            "2026-06-13".to_string(),
            LogLevel::Info,
            "some log message".to_string(),
        );

        smry.count(&entry);
        assert_eq!(smry.total, 1);
        assert_eq!(smry.total_by_level(LogLevel::Info), 1);
        assert_eq!(smry.total_by_level(LogLevel::Debug), 0);

        // Add same

        smry.count(&entry);
        assert_eq!(smry.total, 2);
        assert_eq!(smry.total_by_level(LogLevel::Info), 2);
        assert_eq!(smry.total_by_level(LogLevel::Debug), 0);
    }

    /// Helper: build a Summary from a slice of levels (timestamp/message irrelevant here).
    fn summary_of(levels: &[LogLevel]) -> Summary {
        let mut smry = Summary::default();
        for &level in levels {
            let entry = LogEntry::new("t".to_string(), level, "m".to_string());
            smry.count(&entry);
        }
        smry
    }

    #[test]
    fn empty_summary_is_zero() {
        let smry = Summary::default();
        assert_eq!(smry.total(), 0);
        // Nothing was counted -> iteration yields no levels.
        assert_eq!(smry.iter().count(), 0);
        // Any level absent -> 0, not a panic.
        assert_eq!(smry.total_by_level(LogLevel::Error), 0);
    }

    #[test]
    fn mixed_levels_counted() {
        let smry = summary_of(&[
            LogLevel::Info,
            LogLevel::Error,
            LogLevel::Info,
            LogLevel::Unknown, // boundary: severity 0
        ]);

        assert_eq!(smry.total(), 4);
        assert_eq!(smry.total_by_level(LogLevel::Info), 2);
        assert_eq!(smry.total_by_level(LogLevel::Error), 1);
        assert_eq!(smry.total_by_level(LogLevel::Unknown), 1);
        // Level never seen -> 0.
        assert_eq!(smry.total_by_level(LogLevel::Debug), 0);
    }

    #[test]
    fn levels_iter_sorted_by_severity() {
        // Inserted out of order; BTreeMap must yield them by LogLevel::Ord (severity).
        let smry = summary_of(&[LogLevel::Fatal, LogLevel::Debug, LogLevel::Info]);

        let order: Vec<LogLevel> = smry.iter().map(|(level, _)| level).collect();
        assert_eq!(
            order,
            vec![LogLevel::Debug, LogLevel::Info, LogLevel::Fatal]
        );
    }
}
