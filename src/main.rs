use std::path::PathBuf;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    /// path
    #[clap(short, long, value_parser)]
    path: PathBuf,

    /// search term (regex)
    #[clap(short, long, value_parser)]
    search_term: String,

    /// modified time in the past in min
    #[clap(short, long, value_parser)]
    mins: i64,

    /// prints only the filename with a match
    #[clap(short, long)]
    file_only: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let now = Utc::now();

    let time_past = now.checked_sub_signed(Duration::minutes(cli.mins)).unwrap();

    let re = Regex::new(&cli.search_term).expect("Error with regex");

    for entry in WalkDir::new(&cli.path) {
        let entry = entry?;

        let metadata = entry.metadata()?;

        let modified = metadata.modified()?;

        let modified_utc: DateTime<Utc> = modified.into();

        if metadata.is_file() && modified_utc > time_past {
            let path = entry.path();
            let contents = fs::read_to_string(path)?;

            let mut printed = false;

            for (index, item) in contents.lines().enumerate() {
                if re.is_match(item) {
                    if !printed {
                        println!(
                            "\n{} {:?}",
                            &entry.path().to_str().unwrap().to_string().green(),
                            modified_utc
                        );
                        printed = true;
                    }
                    if !cli.file_only {
                        println!("{}. {}", index, item)
                    }
                }
            }
        }
    }

    Ok(())
}
