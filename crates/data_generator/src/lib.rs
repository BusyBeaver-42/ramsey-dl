#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ramsey_theory::{SequenceProblem, assert_const_generics::*, problems::Schur};
use std::path::PathBuf;

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
