#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod problems;
pub mod sequence_coloring;

pub type Array2D<const N_ROWS: usize, const N_COLUMNS: usize, T> = [[T; N_COLUMNS]; N_ROWS];
