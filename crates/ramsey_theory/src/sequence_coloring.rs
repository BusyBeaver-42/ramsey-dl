use crate::problems::{Array2D, PlayError, SequenceProblem};
use rand::{Rng, seq::SliceRandom};
use std::{cmp, marker::PhantomData};

// TODO: struct
type Coloring = Vec<usize>;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct SequenceColoring<const N_COLORS: usize, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    size: usize,
    partition: Array2D<N_COLORS, { P::BOUND }, bool>,
    possible: Array2D<N_COLORS, { P::BOUND }, bool>,
    _problem: PhantomData<P>,
}

impl<const N_COLORS: usize, P> SequenceColoring<N_COLORS, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    pub const fn new() -> Self {
        Self {
            size: 0,
            partition: [[false; P::BOUND]; N_COLORS],
            possible: [[true; P::BOUND]; N_COLORS],
            _problem: PhantomData,
        }
    }

    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn is_full(&self) -> bool {
        self.size == P::BOUND - 1
    }

    pub fn play(&mut self, color: usize) -> Result<(), PlayError> {
        // TODO: generic over problem
        if self.size >= P::BOUND {
            return Err(PlayError::LimitReached);
        }
        if !self.possible[self.size][color] {
            return Err(PlayError::IllegalMove);
        }

        self.partition[color][self.size] = true;
        self.size += 1;

        let max_updated = cmp::min(2 * self.size, P::BOUND);
        let max_updater = max_updated - self.size;

        let dst = &mut self.possible[color][self.size..max_updated];
        let src = &self.partition[color][..max_updater];

        dst.iter_mut().zip(src).for_each(|(a, &b)| *a &= !b);

        Ok(())
    }

    pub fn legal_moves(&self) -> Vec<usize> {
        (0..N_COLORS)
            .filter(|&color| self.possible[color][self.size])
            .collect()
    }

    pub fn random_move<R>(&self, rng: &mut R) -> Option<usize>
    where
        R: Rng + ?Sized,
    {
        self.legal_moves().choose(rng).copied()
    }

    // TODO: move to Coloring struct
    pub fn random_coloring<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let mut coloring = Self::new();

        while let Some(color) = coloring.random_move(rng) {
            // if random_move returns Some(color) then it is a legal move so this should not happen
            coloring.play(color).expect("Illegal move.");
        }

        coloring
    }

    // TODO: inconsistent, move to Coloring struct
    pub fn random_partial_coloring<R>(rng: &mut R) -> Coloring
    where
        R: Rng + ?Sized,
    {
        let coloring = Self::random_coloring(rng);
        let size = rng.gen_range(0..coloring.size);

        let mut res = vec![0; size];

        for color in 0..N_COLORS {
            for num in 0..size {
                if coloring.partition[color][num] {
                    res[num] = color;
                }
            }
        }

        // TODO: sort in Coloring struct
        let mut colors_seen = 0;
        // TODO: meh, Option?
        let mut color_order = [N_COLORS + 1; N_COLORS];
        for &color in res.iter() {
            if color_order[color] == N_COLORS + 1 {
                color_order[color] = colors_seen;
                colors_seen += 1;

                if colors_seen == N_COLORS {
                    break;
                }
            }
        }

        res.iter_mut().for_each(|color| {
            *color = color_order[*color];
        });

        res
    }
}

impl<const N_COLORS: usize, P> Default for SequenceColoring<N_COLORS, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    fn default() -> Self {
        Self::new()
    }
}
