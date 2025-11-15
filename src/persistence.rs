use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
};

const PROGRESS_FILE: &str = "install_progress.txt";

pub fn load_progress() -> HashSet<String> {
    if let Ok(contents) = fs::read_to_string(PROGRESS_FILE) {
        contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect()
    } else {
        HashSet::new()
    }
}

pub fn save_progress(progress: &HashSet<String>) -> io::Result<()> {
    let mut records: Vec<_> = progress.iter().collect();
    records.sort();
    let mut file = File::create(PROGRESS_FILE)?;
    for entry in records {
        writeln!(file, "{entry}")?;
    }
    Ok(())
}
