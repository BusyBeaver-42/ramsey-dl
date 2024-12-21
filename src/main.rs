#![feature(generic_const_exprs)]

enum Assert<const CHECK: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

trait UpperBound {}

trait SequenceProblem: UpperBound {
    const N_COLORS: usize;

    fn play();
}

struct Schur<const N_COLORS: usize>
where
    Self: UpperBound;

impl UpperBound for Schur<1> {}

impl<const N_COLORS: usize> SequenceProblem for Schur<N_COLORS>
where
    Self: UpperBound,
{
    const N_COLORS: usize = N_COLORS;

    fn play() {}
}

fn run_<P>()
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::N_COLORS]:,
{
}

trait Run {
    fn run(&self);
}

impl<P> Run for P
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::N_COLORS]:,
{
    fn run(&self) {
        run_::<P>()
    }
}

fn main() {
    Box::new(Schur::<1>).run();
}
