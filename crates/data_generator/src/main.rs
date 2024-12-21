#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ramsey_theory::{SequenceProblem, assert_const_generics::*, problems::Schur};

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

pub fn problem_builder(n: usize) -> Box<dyn Run> {
    if n.is_power_of_two() {
        Box::new(Schur::<4>)
    } else {
        Box::new(Schur::<5>)
    }
}

fn main() {
    problem_builder(42).run();
}
