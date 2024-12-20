use crate::{
    cli::Problem as CliProblem, coloring_generation::generate_colorings,
    label_generation::generate_labels, save_data::save_data,
};
use chrono::Local;
use ramsey_theory::{
    SequenceProblem,
    assert_const_generics::*,
    problems::{Schur, WeakSchur},
};
use std::path::PathBuf;

pub type RunFn = fn(Option<PathBuf>, usize, Option<usize>, usize);

macro_rules! get_run_fn {
    ($params:ident, $problem:tt, { $($cases:literal)* }) => {
        {let fn_array = [$(run::<$problem<$cases>>),*];
        let cases = [$($cases),*];
        let index = cases.iter().position(|x| *x == $params).unwrap();
        fn_array[index]}
    };
}

pub fn get_run_fn_pointer(problem: CliProblem, n_colors: usize) -> RunFn {
    match problem {
        CliProblem::Schur => get_run_fn!(n_colors, Schur, { 2 3 4 5 }),
        CliProblem::WeakSchur => get_run_fn!(n_colors, WeakSchur, { 2 3 4 5 }),
        _ => unimplemented!(),
    }
}

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
pub fn run<P>(
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
    let mut output_filename =
        output_filename.unwrap_or_else(|| Local::now().format("%Y%m%d-%H%M%S").to_string().into());
    output_filename.set_extension("npz");

    let n_workers = n_workers.unwrap_or_else(num_cpus::get_physical);

    let colorings = generate_colorings::<P>(n_samples, n_workers, generation_chunk_size);
    let (colorings, sizes, legal_moves) = generate_labels::<P>(colorings);

    save_data(output_filename, colorings, sizes, legal_moves)
}
