#!/usr/bin/env -S cargo +nightly -q -Zscript run --release --manifest-path

---
[package]
version = "0.0.1"
edition = "2021"

[dependencies]
csv = "1.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
---

use std::{collections::HashMap, env, error::Error, fs::File, io::Write};
use serde::{Serialize, Deserialize};
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "開講年度")]
    year: String,
    #[serde(rename = "科目番号")]
    subject_id: String,
    #[serde(rename = "科目名 ")]
    subject_name: String,
    #[serde(rename = "開講区分")]
    subject_type: String,
    #[serde(rename = "科目区分")]
    subject_category: String,
    #[serde(rename = "単位数")]
    credits: String,
    #[serde(rename = "総合評価")]
    grade: String,
}

#[derive(Debug, Serialize)]
struct Subject {
    id: String,
    name: String,
    credits: String,
    r#type: String,
    category: String,
    grade: String,
}

fn convert(input: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(input)?;
    let mut subjects_by_year = HashMap::<String, Vec<Subject>>::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        let subject = Subject {
            id: record.subject_id,
            name: record.subject_name,
            credits: record.credits,
            r#type: record.subject_type,
            category: record.subject_category,
            grade: record.grade,
        };
        subjects_by_year
            .entry(record.year)
            .or_insert_with(Vec::new)
            .push(subject);
    }

    for (year, subjects) in subjects_by_year {
        let json_output = serde_json::to_string_pretty(&subjects)?;
        let filename = format!("{}.json", year);
        let mut file = File::create(&filename)?;
        file.write_all(json_output.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <CSV file>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];

    if let Err(err) = convert(input) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
