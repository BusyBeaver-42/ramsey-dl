#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod assert_const_generics;
pub mod colorings;
pub mod problems;

pub type Array2D<const N_ROWS: usize, const N_COLUMNS: usize, T> = [[T; N_COLUMNS]; N_ROWS];

pub use colorings::{
    coloring::Coloring,
    compressed_coloring::{CompressedColoring, CompressedColors},
    sequence_coloring::{PlayError, SequenceColoring},
};
pub use problems::SequenceProblem;
