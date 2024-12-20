use super::coloring::Coloring;
use ndarray::Array1;
use std::iter;
pub type CompressedColors = u32;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct CompressedColoring<const N_COLORS: usize> {
    compressed: Vec<CompressedColors>,
    size: usize,
}

impl<const N_COLORS: usize> CompressedColoring<N_COLORS> {
    pub const COLORS_PER_ELEM: usize = if N_COLORS.is_power_of_two() {
        (CompressedColors::BITS / N_COLORS.ilog2()) as usize
    } else {
        1 + CompressedColors::MAX.ilog(N_COLORS as CompressedColors) as usize
    };

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn compressed_len(&self) -> usize {
        self.compressed.len()
    }

    pub fn decompress(&self) -> impl Iterator<Item = usize> {
        (0..self.size).map(|num| {
            let div = num / Self::COLORS_PER_ELEM;
            let rem = num % Self::COLORS_PER_ELEM;

            ((self.compressed[div] / (Self::COLORS_PER_ELEM as CompressedColors).pow(rem as u32))
                % (Self::COLORS_PER_ELEM as CompressedColors)) as usize
        })
    }

    pub fn pad_to(&mut self, len: usize) {
        if self.compressed.len() < len {
            let count = len - self.compressed.len();
            self.compressed.extend(iter::repeat_n(0, count));
        }
    }

    pub fn pad_to_longest(colorings: &mut [Self]) {
        let max_len = colorings
            .iter()
            .map(|coloring| coloring.compressed_len())
            .max()
            .expect("The slice is empty");

        colorings
            .iter_mut()
            .for_each(|coloring| coloring.pad_to(max_len));
    }
}

impl<const N_COLORS: usize> From<Coloring<N_COLORS>> for CompressedColoring<N_COLORS> {
    fn from(coloring: Coloring<N_COLORS>) -> Self {
        let compressed = coloring
            .chunks(Self::COLORS_PER_ELEM)
            .map(|c| {
                c.iter().rev().fold(0, |acc, &color| {
                    N_COLORS as CompressedColors * acc + color as CompressedColors
                })
            })
            .collect();

        Self {
            compressed,
            size: coloring.len(),
        }
    }
}

impl<const N_COLORS: usize> From<CompressedColoring<N_COLORS>> for Array1<CompressedColors> {
    fn from(coloring: CompressedColoring<N_COLORS>) -> Self {
        Array1::from(coloring.compressed)
    }
}
