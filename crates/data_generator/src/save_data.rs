use ndarray::{Array1, Array2};
use ndarray_npy::NpzWriter;
use std::fs::File;

pub fn save_data(
    filename: &str,
    colorings: Array2<u32>,
    sizes: Array1<u32>,
    legal_moves: Array2<bool>,
) {
    let file = File::create(filename).unwrap();
    let mut npz = NpzWriter::new_compressed(file);

    npz.add_array("colorings", &colorings).unwrap();
    npz.add_array("sizes", &sizes).unwrap();
    npz.add_array("legal_moves", &legal_moves).unwrap();

    npz.finish().unwrap();
}
