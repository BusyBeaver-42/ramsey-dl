use crate::Array2D;

pub trait UpperBound {
    const BOUND: usize;
}

pub trait SequenceProblem: UpperBound {
    const N_COLORS: usize;

    fn play(
        size: &mut usize,
        partition: &mut Array2D<{ Self::N_COLORS }, { Self::BOUND }, bool>,
        possible: &mut Array2D<{ Self::N_COLORS }, { Self::BOUND }, bool>,
        color: usize,
    );
}

pub struct Schur<const N_COLORS: usize>
where
    Self: UpperBound;

impl UpperBound for Schur<1> {
    const BOUND: usize = 1;
}

impl<const N_COLORS: usize> SequenceProblem for Schur<N_COLORS>
where
    Self: UpperBound,
{
    const N_COLORS: usize = N_COLORS;

    fn play(
        size: &mut usize,
        partition: &mut Array2D<{ Self::N_COLORS }, { Self::BOUND }, bool>,
        possible: &mut Array2D<{ Self::N_COLORS }, { Self::BOUND }, bool>,
        color: usize,
    ) {
    }
}
