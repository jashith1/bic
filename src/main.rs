use clap::{Parser, Subcommand};
use std::{fs, path::Path};

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
}

fn main() -> std::io::Result<()>{
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init_bic()?;
        }
    }

    Ok(())
}


fn init_bic() -> std::io::Result<()> {
    if Path::new(".bic").exists() {
        eprintln!("bic repository already exists.");
        std::process::exit(1);
    }
    fs::create_dir(".bic")?;
    println!("Initialized empty bic repository.");
    Ok(())
}