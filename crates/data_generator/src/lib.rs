#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use chrono::Local;
use coloring_generation::generate_colorings;
use label_generation::generate_labels;
use ramsey_theory::{SequenceProblem, assert_const_generics::*, problems::Schur};
use save_data::save_data;
use std::path::PathBuf;

mod coloring_generation;
mod label_generation;
mod save_data;

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
fn run_<P>(
    output_filename: Option<PathBuf>,
    n_samples: usize,
    n_workers: Option<usize>,
    generation_chunk_size: usize,
) where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let output_filename =
        output_filename.unwrap_or_else(|| Local::now().format("%Y%m%d-%H%M%S").to_string().into());

    let n_workers = n_workers.unwrap_or_else(num_cpus::get_physical);

    let colorings = generate_colorings::<P>(n_samples, n_workers, generation_chunk_size);
    let (colorings, sizes, legal_moves) = generate_labels::<P>(colorings);

    save_data(output_filename, colorings, sizes, legal_moves)
}

pub trait Run {
    fn run(
        &self,
        output_filename: Option<PathBuf>,
        n_samples: usize,
        n_workers: Option<usize>,
        generation_chunk_size: usize,
    );
}

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
impl<P> Run for P
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    fn run(
        &self,
        output_filename: Option<PathBuf>,
        n_samples: usize,
        n_workers: Option<usize>,
        generation_chunk_size: usize,
    ) {
        run_::<P>(output_filename, n_samples, n_workers, generation_chunk_size)
    }
}

pub fn problem_builder(n: usize) -> Box<dyn Run> {
    if n.is_power_of_two() {
        Box::new(Schur::<4>)
    } else {
        Box::new(Schur::<5>)
    }
}
