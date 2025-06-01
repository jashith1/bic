use clap::{Parser, Subcommand};
use std::{fs, path::Path};
mod commit;
mod log;
pub mod util;
mod reset;

#[derive(Parser)]
#[command(name = "bic")]
#[command(about = "A simple version control system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Log,
    Reset {
        #[arg(value_name = "COMMIT_HASH")]
        commit_hash: String,
    },
}

fn main() -> std::io::Result<()>{
    let cli = Cli::parse();

    //find which command is used
    match cli.command {
        Commands::Init => init_bic()?,
        Commands::Commit {message} => commit::commit(message)?,
        Commands::Log => log::log()?,
        Commands::Reset {commit_hash} => reset::reset(commit_hash)?,
    }

    Ok(())
}


fn init_bic() -> std::io::Result<()> {
    //if already initialized, skip
    if Path::new(".bic").exists() {
        eprintln!("bic repository already exists.");
        std::process::exit(1);
    }

    //create required folders
    fs::create_dir(".bic")?;
    fs::create_dir(".bic/objects")?;
    fs::create_dir(".bic/commits")?;
    fs::write(".bic/HEAD", "null")?;
    // fs::write(".bic_ignore", "")?;
    println!("Initialized empty bic repository.");
    Ok(())
}

