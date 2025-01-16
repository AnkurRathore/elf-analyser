mod elf_parser;
mod utils;
use clap::Parser;

#[derive(Parser)]
#[command(name = "elf_parser", version = "0.1", about = "A tool to analyse ELF files")]
struct Args{
    /// Path to ELF file
    #[arg(short, long)]
    file : String,
}

fn main() {
    let args = Args::parse();

    match std::fs::read(&args.file) {
        Ok(data) => {
            if let Err(e) = elf_parser::analyze_elf(&data) {
                eprintln!("Error parsing ELF file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
