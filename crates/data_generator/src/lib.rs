#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use chrono::Local;
use coloring_generation::generate_colorings;
use label_generation::generate_labels;
use ramsey_theory::SequenceProblem;
use save_data::save_data;
use std::path::PathBuf;

mod coloring_generation;
mod label_generation;
mod save_data;

pub fn run<const N_COLORS: usize, P>(
    output_filename: Option<PathBuf>,
    n_samples: usize,
    n_workers: Option<usize>,
    generation_chunk_size: usize,
) where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    let output_filename =
        output_filename.unwrap_or_else(|| Local::now().format("%Y%m%d-%H%M%S").to_string().into());

    let n_workers = n_workers.unwrap_or_else(num_cpus::get_physical);

    let colorings = generate_colorings::<N_COLORS, P>(n_samples, n_workers, generation_chunk_size);
    let (colorings, sizes, legal_moves) = generate_labels::<N_COLORS, P>(colorings);

    save_data(output_filename, colorings, sizes, legal_moves)
}
