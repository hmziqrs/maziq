use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{catalog::SoftwareId, manager::ActionKind};

const HISTORY_FILE: &str = "install_history.jsonl";

#[derive(Serialize, Deserialize)]
pub struct HistoryRecord {
    pub software: String,
    pub action: String,
    pub version: Option<String>,
    pub source: Option<String>,
    pub timestamp: u64,
}

impl HistoryRecord {
    pub fn new(
        id: SoftwareId,
        action: ActionKind,
        version: Option<String>,
        source: Option<String>,
    ) -> Self {
        Self {
            software: id.key().to_string(),
            action: action.label().to_string(),
            version,
            source,
            timestamp: current_timestamp(),
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn append(record: &HistoryRecord) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)?;
    serde_json::to_writer(&mut file, record)?;
    file.write_all(b"\n")?;
    Ok(())
}
