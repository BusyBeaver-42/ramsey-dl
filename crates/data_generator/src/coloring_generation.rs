use ramsey_theory::{Coloring, CompressedColoring, SequenceProblem, assert_const_generics::*};
use rand::{Rng, thread_rng};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
fn coloring_generation<P, R>(
    n_samples: usize,
    rng: &mut R,
) -> HashSet<CompressedColoring<{ P::N_COLORS }>>
where
    R: Rng + ?Sized,
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let mut colorings = HashSet::with_capacity(n_samples);

    while colorings.len() < n_samples {
        let coloring = Coloring::<{ P::N_COLORS }>::random_partial::<P, _>(rng);
        let compressed = CompressedColoring::from(coloring);
        colorings.insert(compressed);
    }

    colorings
}

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
fn mpsc_coloring_generator<P>(
    n_samples: usize,
    n_workers: usize,
    chunk_size: usize,
) -> HashSet<CompressedColoring<{ P::N_COLORS }>>
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let mut colorings = HashSet::with_capacity(n_samples + n_workers * chunk_size);

    let keep_running = AtomicBool::new(true);

    thread::scope(|scope| {
        let (tx, rx) = mpsc::channel();

        for _ in 1..n_workers {
            let keep_running = &keep_running;
            let tx = tx.clone();

            scope.spawn(move || {
                let mut rng = thread_rng();

                while keep_running.load(Ordering::Acquire) {
                    let colorings = coloring_generation::<P, _>(chunk_size, &mut rng);
                    tx.send(colorings).unwrap();
                }
            });
        }

        // Drop the last sender to stop `rx` waiting for message.
        drop(tx);

        let mut signal_not_sent = true;

        while let Ok(received) = rx.recv() {
            colorings.extend(received);

            if signal_not_sent && colorings.len() >= n_samples {
                keep_running.store(false, Ordering::Release);
                signal_not_sent = false;
            }
        }
    });

    colorings
}

// Clippy false positive: rustc needs `P::N_COLORS == P::N_COLORS`
#[allow(clippy::eq_op)]
pub fn generate_colorings<P>(
    n_samples: usize,
    n_workers: usize,
    chunk_size: usize,
) -> Vec<CompressedColoring<{ P::N_COLORS }>>
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let colorings = if n_workers <= 1 {
        coloring_generation::<P, _>(n_samples, &mut thread_rng())
    } else {
        mpsc_coloring_generator::<P>(n_samples, n_workers, chunk_size)
    };

    colorings.into_iter().collect()
}
