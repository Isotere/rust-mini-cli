use std::collections::BTreeMap;

use crate::entry::LogLevel;

#[derive(Debug, Default)]
pub struct Summary {
    pub total: usize,
    pub by_level: BTreeMap<LogLevel, usize>,
}
