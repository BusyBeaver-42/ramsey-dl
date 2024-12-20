use crate::{Array2D, problems::SequenceProblem};
use rand::{Rng, seq::SliceRandom};
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum PlayError {
    InvalidColor,
    LimitReached,
    IllegalMove,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct SequenceColoring<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    size: usize,
    partition: Array2D<{ P::N_COLORS }, { P::BOUND }, bool>,
    possible: Array2D<{ P::N_COLORS }, { P::BOUND }, bool>,
    _problem: PhantomData<P>,
}

impl<P> SequenceColoring<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    pub const fn new() -> Self {
        Self {
            size: 0,
            partition: [[false; P::BOUND]; P::N_COLORS],
            possible: [[true; P::BOUND]; P::N_COLORS],
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
        if color >= P::N_COLORS {
            return Err(PlayError::InvalidColor);
        }
        if self.size >= P::BOUND {
            return Err(PlayError::LimitReached);
        }
        if !self.possible[self.size][color] {
            return Err(PlayError::IllegalMove);
        }

        P::play(
            &mut self.size,
            &mut self.partition,
            &mut self.possible,
            color,
        );

        Ok(())
    }

    pub fn legal_moves(&self) -> Vec<usize> {
        (0..P::N_COLORS)
            .filter(|&color| self.possible[color][self.size])
            .collect()
    }

    pub fn random_move<R>(&self, rng: &mut R) -> Option<usize>
    where
        R: Rng + ?Sized,
    {
        self.legal_moves().choose(rng).copied()
    }
}

impl<P> Default for SequenceColoring<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> IntoIterator for SequenceColoring<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    type Item = usize;
    type IntoIter = SequenceColoringIntoIter<P>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            size: self.size,
            num: 0,
            partition: self.partition,
        }
    }
}

pub struct SequenceColoringIntoIter<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    size: usize,
    num: usize,
    partition: Array2D<{ P::N_COLORS }, { P::BOUND }, bool>,
}

impl<P> Iterator for SequenceColoringIntoIter<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == self.size {
            return None;
        }

        let color = (0..P::N_COLORS).find(|&color| self.partition[color][self.num]);
        self.num += 1;

        color
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.num;

        (remaining, Some(remaining))
    }
}

impl<P> ExactSizeIterator for SequenceColoringIntoIter<P>
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
}
