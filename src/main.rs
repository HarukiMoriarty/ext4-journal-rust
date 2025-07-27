use clap::{Parser, Subcommand};
use ext4fs::FileSystem;
use std::io::Result;

#[derive(Parser)]
#[command(name = "ext4fs")]
#[command(about = "Explore an ext4 filesystem image")]
struct Cli {
    /// Path to ext4 image (default: ext4.img)
    #[arg(short, long, default_value = "ext4.img")]
    image: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read and print file content
    ReadFile {
        /// Absolute path to file
        path: String,
    },
    /// List directory entries
    ListDir {
        /// Absolute path to directory
        path: String,
    },
    /// Print inode metadata
    Stat {
        /// Absolute path to file or directory
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut fs = FileSystem::open(&cli.image)?;

    match cli.command {
        Commands::ReadFile { path } => {
            let content = fs.read_file(&path)?;
            println!("{}", String::from_utf8_lossy(&content));
        }
        Commands::ListDir { path } => todo!("Implement list directory command"),
        Commands::Stat { path } => todo!("Implement stat command"),
    }

    Ok(())
}
