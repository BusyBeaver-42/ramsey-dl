#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ramsey_theory::{Coloring, CompressedColoring, SequenceColoring, SequenceProblem};
use rand::{Rng, thread_rng};
use std::collections::HashSet;

fn generate_partial_colorings<const N_COLORS: usize, P, R>(
    n_samples: usize,
    rng: &mut R,
) -> Vec<CompressedColoring<N_COLORS>>
where
    R: Rng + ?Sized,
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    let mut colorings = HashSet::with_capacity(n_samples);

    while colorings.len() < n_samples {
        let coloring = Coloring::<N_COLORS>::random_partial::<P, _>(rng);
        let compressed = CompressedColoring::from(coloring);
        colorings.insert(compressed);
    }

    colorings.into_iter().collect()
}

fn generate_labels<const N_COLORS: usize, P>(
    colorings: &[CompressedColoring<N_COLORS>],
) -> Vec<[bool; N_COLORS]>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    colorings
        .iter()
        .map(|compressed| {
            let mut coloring = SequenceColoring::<N_COLORS, P>::new();
            for color in compressed.decompress() {
                coloring.play(color).unwrap();
            }

            let mut legal = [false; N_COLORS];

            for color in coloring.legal_moves() {
                legal[color] = true;
            }

            legal
        })
        .collect()
}

fn run<P>(n_workers: Option<usize>, n_samples: usize) {
    let n_workers = n_workers.unwrap_or_else(num_cpus::get_physical);

    if n_workers == 1 {
        unimplemented!();
    } else {
        unimplemented!();
    }
}
