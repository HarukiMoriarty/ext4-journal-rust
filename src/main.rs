use clap::Parser;
use ext4fs::FileSystem;
use std::io::{self, BufRead, Write};

/// ext4fs interactive explorer
#[derive(Parser)]
#[command(name = "ext4fs")]
#[command(about = "Explore an ext4 filesystem image interactively")]
struct Cli {
    /// Path to ext4 image
    #[arg(short, long, default_value = "ext4.img")]
    image: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut fs = FileSystem::open(&cli.image)?;
    println!("Opened image: {}", cli.image);
    println!("Type 'help' for available commands. Type 'exit' to quit.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" | "quit" => break,
            "help" => {
                println!("Commands:");
                println!("  read <path>   - Read and print file content");
                println!("  ls <path>     - List directory entries");
                println!("  stat <path>   - Print inode metadata (TODO)");
                println!("  exit, quit    - Exit the interactive shell");
            }
            "read" if args.len() == 2 => match fs.read_file(args[1]) {
                Ok(content) => println!("{}", String::from_utf8_lossy(&content)),
                Err(e) => eprintln!("Error reading file: {e}"),
            },
            "ls" if args.len() == 2 => match fs.list_dir(args[1]) {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry);
                    }
                }
                Err(e) => eprintln!("Error listing directory: {e}"),
            },
            "stat" => {
                println!("TODO: '{}' command is not implemented yet.", args[0]);
            }
            _ => {
                eprintln!("Unknown or malformed command. Type 'help' for available commands.");
            }
        }
    }

    Ok(())
}
