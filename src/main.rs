use std::path::PathBuf;

use chrono::{DateTime, Utc, Duration};
use clap::{Parser};
use walkdir::WalkDir;
use anyhow::Result;
use colored::*;
use std::fs;


#[derive(Parser)]
struct Cli {
    /// path
    #[clap(short, long, value_parser)]
    path: PathBuf,

    /// search term
    #[clap(short, long, value_parser)]
    search_term: String,

    /// modified time in the past in min
    #[clap(short, long, value_parser)]
    mins: i64,

    /// case insensitive
    #[clap(short, long)]
    insensitive: bool
}

fn main() -> Result<()>{

    let cli = Cli::parse();

    let now = Utc::now();

    let time_past = now.checked_sub_signed(Duration::minutes(cli.mins)).unwrap();
    

    for entry in WalkDir::new(&cli.path) {
        let entry = entry?;

        let metadata = entry.metadata()?;

        let modified = metadata.modified()?;

        let modified_utc : DateTime<Utc> = modified.into();

        if metadata.is_file() && modified_utc > time_past {
            let path = entry.path();
            let contents = fs::read_to_string(path)?;

            println!("\n{} {:?}", &entry.path().to_str().unwrap().to_string().green(), modified_utc);
            for (index, item) in contents.lines().enumerate() {

                let st: String;
                let it: String;

                if cli.insensitive {
                    st = cli.search_term.to_lowercase();
                    it = item.to_lowercase();
                } else {
                    st = cli.search_term.to_string();
                    it = item.to_string();
                }
                if it.contains(&st) {
                    println!("{}. {}", index, item)
                }
            }
        }
    }

    Ok(())
}
