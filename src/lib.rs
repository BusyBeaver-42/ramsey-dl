pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait UpperBound {
    const BOUND: usize;
}

pub trait SequenceProblem: UpperBound {
    const N_COLORS: usize;

    fn play();
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

    fn play() {}
}
