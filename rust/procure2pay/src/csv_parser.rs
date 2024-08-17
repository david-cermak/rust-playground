use chrono::NaiveDateTime;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_csv(file_path: &str) -> Result<Vec<(String, NaiveDateTime, String)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut cases = Vec::new();
    let mut first_line = true;
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(';').collect();

        if first_line {
            first_line = false;
            continue;
        }
        if fields.len() != 3 {
            continue; // Skip malformed lines
        }

        let case_id = fields[0].to_string();
        let timestamp = NaiveDateTime::parse_from_str(fields[2], "%Y-%m-%d %H:%M:%S%.f")?;
        let activity_name = fields[1].to_string();

        cases.push((case_id, timestamp, activity_name));
    }

    Ok(cases)
}
