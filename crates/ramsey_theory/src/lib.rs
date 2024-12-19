#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod assert_const_generics {
    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}
}

pub mod colorings;
pub mod problems;

pub type Array2D<const N_ROWS: usize, const N_COLUMNS: usize, T> = [[T; N_COLUMNS]; N_ROWS];

pub use colorings::{
    coloring::Coloring,
    compressed_coloring::CompressedColoring,
    sequence_coloring::{PlayError, SequenceColoring},
};
pub use problems::SequenceProblem;
