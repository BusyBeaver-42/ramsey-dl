use assert_const_generics::*;
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
    ($({$($generics:tt)+})? $problem:ty = $value:expr $(; where $($generics_bounds:tt)+)?) => {
        impl$(<$($generics)+>)? $crate::problems::UpperBound for $problem
        $(where $($generics_bounds)+)?
        {
            const BOUND: usize = $value + 1;
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, Default)]
pub struct WeakSchur<const N_COLORS: usize>
where
    Self: UpperBound;

upper_bound_impl! { WeakSchur<1> = 2 }
upper_bound_impl! { WeakSchur<2> = 8 }
upper_bound_impl! { WeakSchur<3> = 23 }
upper_bound_impl! { WeakSchur<4> = 66 }
upper_bound_impl! { WeakSchur<5> = 200 }

impl<const N_COLORS: usize> SequenceProblem<N_COLORS> for WeakSchur<N_COLORS>
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

        let max_updated = cmp::min(2 * *size - 1, Self::BOUND);
        let max_updater = max_updated - *size;

        let dst = &mut possible[color][*size..max_updated];
        let src = &partition[color][..max_updater];

        dst.iter_mut().zip(src).for_each(|(a, &b)| *a &= !b);
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, Default)]
pub struct VanDerWaerden<const N_COLORS: usize, const PROGRESSION_LEN: usize>
where
    Self: UpperBound;

upper_bound_impl! {
    { const PROGRESSION_LEN: usize } VanDerWaerden<1, PROGRESSION_LEN> = PROGRESSION_LEN - 1 ;
    where
        Assert<{ PROGRESSION_LEN > 1 }>: IsTrue,
}
upper_bound_impl! {
    { const N_COLORS: usize } VanDerWaerden<N_COLORS, 2> = N_COLORS ;
    where
        Assert<{ N_COLORS > 1 }>: IsTrue,
}
upper_bound_impl! { VanDerWaerden<2, 3> = 8 }
upper_bound_impl! { VanDerWaerden<2, 4> = 34 }
upper_bound_impl! { VanDerWaerden<2, 5> = 177 }
upper_bound_impl! { VanDerWaerden<3, 3> = 26 }
upper_bound_impl! { VanDerWaerden<3, 4> = 292 }
upper_bound_impl! { VanDerWaerden<4, 3> = 75 }
upper_bound_impl! { VanDerWaerden<5, 3> = 180 }
upper_bound_impl! { VanDerWaerden<6, 3> = 242 }

impl<const PROGRESSION_LEN: usize, const N_COLORS: usize> SequenceProblem<N_COLORS>
    for VanDerWaerden<PROGRESSION_LEN, N_COLORS>
where
    Self: UpperBound,
{
    fn play(
        size: &mut usize,
        partition: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        possible: &mut Array2D<N_COLORS, { Self::BOUND }, bool>,
        color: usize,
    ) {
        let _ignore = (size, partition, possible, color);
        todo!();
    }
}

mod assert_const_generics {
    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}
}
