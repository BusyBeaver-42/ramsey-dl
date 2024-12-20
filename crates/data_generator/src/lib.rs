#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod cli;
mod coloring_generation;
mod label_generation;
mod run;
mod save_data;

pub use cli::Cli;
pub use run::{get_run_fn_pointer, run};
