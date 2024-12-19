use std::cmp;

pub type Array2D<const N_ROWS: usize, const N_COLUMNS: usize, T> = [[T; N_COLUMNS]; N_ROWS];

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum PlayError {
    LimitReached,
    IllegalMove,
}

pub trait UpperBound {
    const BOUND: usize;
}

#[macro_export]
macro_rules! upper_bound_impl {
    ($problem:ty = $bound:literal) => {
        impl $crate::problems::UpperBound for $problem {
            const BOUND: usize = $bound + 1;
        }
    };
}

pub trait SequenceProblem<const N_COLORS: usize>: UpperBound {
    fn play(
        size: &mut usize,
        partition: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        possible: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        color: usize,
    );
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, Default)]
pub struct Schur<const N_COLORS: usize>
where
    Self: UpperBound;

upper_bound_impl! { Schur<1> = 1 }
upper_bound_impl! { Schur<2> = 4 }
upper_bound_impl! { Schur<3> = 13 }
upper_bound_impl! { Schur<4> = 44 }
upper_bound_impl! { Schur<5> = 160 }

impl<const N_COLORS: usize> SequenceProblem<N_COLORS> for Schur<N_COLORS>
where
    Self: UpperBound,
{
    fn play(
        size: &mut usize,
        partition: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        possible: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        color: usize,
    ) {
        partition[color][*size] = true;
        *size += 1;

        let max_updated = cmp::min(2 * *size, Self::BOUND);
        let max_updater = max_updated - *size;

        let dst = &mut possible[color][*size..max_updated];
        let src = &partition[color][..max_updater];

        dst.iter_mut().zip(src).for_each(|(a, &b)| *a &= !b);
    }
}
