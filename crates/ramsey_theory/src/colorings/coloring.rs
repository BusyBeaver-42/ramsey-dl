use super::sequence_coloring::SequenceColoring;
use crate::problems::SequenceProblem;
use rand::Rng;
use std::ops::Deref;

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

impl<const N_COLORS: usize, P> From<SequenceColoring<N_COLORS, P>> for Coloring<N_COLORS>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    fn from(coloring: SequenceColoring<N_COLORS, P>) -> Self {
        coloring.into_iter().collect()
    }
}
