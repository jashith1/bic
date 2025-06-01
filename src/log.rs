use crate::util::{read_head::read_current_head,commit_data::CommitData};
use std::{fs, path::Path};
use serde_json;
use chrono::{Local, TimeZone};
use colored::Colorize;

pub fn log() -> std::io::Result<()>{
    if !Path::new(".bic").exists() {
        eprintln!("Error: not bic repository. Run `bic init` first.");
        std::process::exit(1);
    }
    let current_head = read_current_head()?;

    if current_head == "null"{
        println!("No commits yet");
        return Ok(());
    }

    print_commit_details(current_head, true)?;
    Ok(())
}

fn print_commit_details(commit_hash: String, is_head: bool) -> std::io::Result<()>{
    let commit_string = fs::read_to_string(format!(".bic/commits/{}.json", commit_hash))?;
    let commit_data: CommitData = serde_json::from_str(&commit_string)?;

    if is_head{
        println!("Commit: {} {}", commit_hash.yellow(), "(HEAD)".blue());
    } else {
        println!("Commit: {}", commit_hash.yellow());
    }

    println!("Data: {}", unix_to_localtime(commit_data.timestamp));
    println!("Message: {}", commit_data.message);

    if !commit_data.parent.is_empty() && commit_data.parent != "null"{
        print!("\n");
        print_commit_details(commit_data.parent, false)?;
    }

    Ok(())
}

fn unix_to_localtime(timestamp: u64) -> String {
    match Local.timestamp_opt(timestamp as i64, 0) {
        chrono::LocalResult::Single(datetime) => {
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        _ => {
            format!("Invalid timestamp: {}", timestamp)
        }
    }
}