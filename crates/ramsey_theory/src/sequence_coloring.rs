use crate::{Array2D, problems::SequenceProblem};
use rand::{Rng, seq::SliceRandom};
use std::{marker::PhantomData, ops::Deref};

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct Coloring<const N_COLORS: usize>(Vec<usize>);

impl<const N_COLORS: usize> Coloring<N_COLORS> {
    pub fn random<P, R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
        P: SequenceProblem<N_COLORS>,
        [(); P::BOUND]:,
    {
        let mut coloring = SequenceColoring::<N_COLORS, P>::new();

        while let Some(color) = coloring.random_move(rng) {
            // if random_move returns Some(color) then it is a legal move so this should not panic
            coloring.play(color).expect("Illegal move.");
        }

        Self::from(coloring)
    }

    pub fn random_partial<P, R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
        P: SequenceProblem<N_COLORS>,
        [(); P::BOUND]:,
    {
        let mut coloring = Self::random::<P, _>(rng);

        let size = rng.gen_range(0..coloring.len());

        coloring.truncate(size);
        coloring.shrink_to_fit();

        coloring
    }

    pub fn truncate(&mut self, size: usize) {
        self.0.truncate(size);
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    pub fn order_colors(&mut self) {
        let mut colors_seen = 0;
        let mut color_order = [None; N_COLORS];

        for color in self.0.iter_mut() {
            if color_order[*color].is_none() {
                color_order[*color] = Some(colors_seen);
                colors_seen += 1;
            }

            *color = color_order[*color].unwrap();
        }
    }
}

// do not implement DerefMut, otherwise the user could put an invalid color in the Vec
impl<const N_COLORS: usize> Deref for Coloring<N_COLORS> {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N_COLORS: usize> FromIterator<usize> for Coloring<N_COLORS> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        Self(iter.into_iter().collect())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum PlayError {
    InvalidColor,
    LimitReached,
    IllegalMove,
}

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
        if color >= N_COLORS {
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

impl<const N_COLORS: usize, P> IntoIterator for SequenceColoring<N_COLORS, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    type Item = usize;
    type IntoIter = SequenceColoringIntoIter<N_COLORS, P>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            num: 0,
            partition: self.partition,
        }
    }
}

impl<const N_COLORS: usize, P> From<SequenceColoring<N_COLORS, P>> for Coloring<N_COLORS>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    fn from(coloring: SequenceColoring<N_COLORS, P>) -> Self {
        coloring.into_iter().collect()
    }
}

pub struct SequenceColoringIntoIter<const N_COLORS: usize, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    num: usize,
    partition: Array2D<N_COLORS, { P::BOUND }, bool>,
}

impl<const N_COLORS: usize, P> Iterator for SequenceColoringIntoIter<N_COLORS, P>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == P::BOUND {
            return None;
        }

        let color = (0..N_COLORS).find(|&color| self.partition[color][self.num]);
        self.num += 1;

        color
    }
}
