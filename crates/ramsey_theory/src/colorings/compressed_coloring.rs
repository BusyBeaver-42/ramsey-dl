use super::coloring::Coloring;

type CompressedColors = u32;

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

    pub fn decompress(&self) -> impl Iterator<Item = usize> {
        (0..self.size).map(|num| {
            let div = num / Self::COLORS_PER_ELEM;
            let rem = num % Self::COLORS_PER_ELEM;

            ((self.compressed[div] / (Self::COLORS_PER_ELEM as CompressedColors).pow(rem as u32))
                % (Self::COLORS_PER_ELEM as CompressedColors)) as usize
        })
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
