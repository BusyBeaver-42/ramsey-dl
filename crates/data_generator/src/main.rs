use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    colors: u8,

    #[arg(short, long)]
    #[arg(default_value_t = 100_000)]
    samples: usize,

    #[arg(short, long)]
    output_file: Option<PathBuf>,

    #[arg(short, long)]
    workers: Option<usize>,

    #[arg(short, long)]
    #[arg(default_value_t = 500)]
    chunk_size: usize,

    #[arg(short, long, value_enum)]
    problem: Problem,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Problem {
    Schur,
    WeakSchur,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
