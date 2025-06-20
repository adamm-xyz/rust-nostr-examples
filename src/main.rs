use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "rust-nostr-examples")]
#[command(about = "A CLI tool for exploring Nostr NIPs with rust-nostr")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// NIP01: Basic protocol flow (events, keys, relay communication)
    Nip01,
    
    /// NIP02: Contact List and Petnames
    Nip02,
    
    /// NIP03: OpenTimestamps Attestations for Events
    Nip03,
    
    /// NIP04: Encrypted Direct Messages
    Nip04,
    
    /// NIP05: Mapping Nostr keys to DNS-based internet identifiers
    Nip05,
    
    /// NIP06: Basic key derivation from mnemonic seed phrase
    Nip06,
    
    /// List all available NIPs
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Nip01 => run_nip01().await,
        Commands::Nip02 => todo!(),
        Commands::Nip03 => todo!(),
        Commands::Nip04 => todo!(),
        Commands::Nip05 => todo!(),
        Commands::Nip06 => todo!(),
        Commands::List => {
            list_nips();
            Ok(())
        }
    };
    
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

// NIP runner functions
async fn run_nip01() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP01 example...\n");
    rust_nostr_examples::examples::nip01::run().await
}
/*
async fn run_nip02() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP02 example...\n");
    rust_nostr_examples::examples::nip02::run().await
}

async fn run_nip03() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP03 example...\n");
    rust_nostr_examples::examples::nip03::run().await
}

async fn run_nip04() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP04 example...\n");
    rust_nostr_examples::examples::nip04::run().await
}

async fn run_nip05() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP05 example...\n");
    rust_nostr_examples::examples::nip05::run().await
}

async fn run_nip06() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running NIP06 example...\n");
    rust_nostr_examples::examples::nip06::run().await
}
*/

fn list_nips() {
    println!("Available Nostr Improvement Proposals (NIPs):");
    
    println!("  nip01   - Basic protocol flow");
    println!("  nip02   - Contact List and Petnames");
    println!("  nip03   - OpenTimestamps Attestations");
    println!("  nip04   - Encrypted Direct Messages");
    println!("  nip05   - DNS-based identity mapping");
    println!("  nip06   - Basic key derivation from mnemonic");
    
    println!("\nUsage:");
    println!("  cargo run -- <nip>     # Run specific NIP example");
    println!("  cargo run -- list      # Show this list");
    println!("  cargo run -- --help    # Show detailed help");
}
