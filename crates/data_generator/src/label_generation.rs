use ramsey_theory::{CompressedColoring, SequenceColoring, SequenceProblem};

pub fn generate_labels<const N_COLORS: usize, P>(
    colorings: &[CompressedColoring<N_COLORS>],
) -> Vec<[bool; N_COLORS]>
where
    P: SequenceProblem<N_COLORS>,
    [(); P::BOUND]:,
{
    colorings
        .iter()
        .map(|compressed| {
            let mut coloring = SequenceColoring::<N_COLORS, P>::new();
            for color in compressed.decompress() {
                coloring.play(color).unwrap();
            }

            let mut legal = [false; N_COLORS];

            for color in coloring.legal_moves() {
                legal[color] = true;
            }

            legal
        })
        .collect()
}
