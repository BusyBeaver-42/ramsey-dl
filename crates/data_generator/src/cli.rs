use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, value_enum)]
    pub problem: Problem,

    #[arg(short, long)]
    pub colors: usize,

    #[arg(short, long)]
    #[arg(default_value_t = 100_000)]
    pub samples: usize,

    #[arg(short, long)]
    pub output_file: Option<PathBuf>,

    #[arg(short, long)]
    pub workers: Option<usize>,

    #[arg(long)]
    #[arg(default_value_t = 500)]
    pub chunk_size: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Problem {
    Schur,
    WeakSchur,
    VanDerWaerden,
}
