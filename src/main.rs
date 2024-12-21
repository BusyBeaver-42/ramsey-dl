#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use reproduce_crash::{Assert, IsTrue, Schur, SequenceProblem};

fn run_<P>()
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
}

pub trait Run {
    fn run(&self);
}

impl<P> Run for P
where
    P: SequenceProblem,
    Assert<{ P::N_COLORS == P::N_COLORS }>: IsTrue,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    fn run(&self) {
        run_::<P>()
    }
}

fn main() {
    Box::new(Schur::<1>).run();
}
