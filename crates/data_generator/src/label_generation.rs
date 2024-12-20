use ndarray::{Array1, Array2};
use ramsey_theory::{CompressedColoring, CompressedColors, SequenceColoring, SequenceProblem};

fn label_generation<P>(
    colorings: &[CompressedColoring<{ P::N_COLORS }>],
) -> (Vec<u32>, Array2<bool>)
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let mut legal_moves = Array2::from_elem((colorings.len(), P::N_COLORS), false);
    colorings
        .iter()
        .map(|compressed| {
            let mut coloring = SequenceColoring::<P>::new();
            for color in compressed.decompress() {
                coloring.play(color).unwrap();
            }
            coloring.legal_moves()
        })
        .zip(legal_moves.rows_mut())
        .for_each(|(moves, mut row)| {
            for m in moves {
                row[m] = true;
            }
        });

    let sizes = colorings
        .iter()
        .map(|coloring| coloring.size() as u32)
        .collect();

    (sizes, legal_moves)
}

fn nested_to_array2<A, T>(colorings: Vec<A>) -> Array2<T>
where
    A: Into<Array1<T>>,
    T: Copy,
{
    unsafe {
        let n_rows = colorings.len();
        let mut colorings = colorings.into_iter().map(Into::into);

        let first_row = colorings.next().expect("Empty Vec");
        let n_columns = first_row.len();

        let mut arr = Array2::uninit((n_rows, n_columns));
        let mut rows = arr.rows_mut().into_iter();

        first_row.assign_to(rows.next().unwrap());
        rows.zip(colorings)
            .for_each(|(row, coloring)| coloring.assign_to(row));

        // SAFETY: there are `colorings.len()` rows so all the elements have been initialized
        arr.assume_init()
    }
}

pub fn generate_labels<P>(
    mut colorings: Vec<CompressedColoring<{ P::N_COLORS }>>,
) -> (Array2<CompressedColors>, Array1<u32>, Array2<bool>)
where
    P: SequenceProblem,
    [(); P::BOUND]:,
    [(); P::N_COLORS]:,
{
    let (sizes, legal_moves) = label_generation(&colorings);

    CompressedColoring::<{ P::N_COLORS }>::pad_to_longest(&mut colorings);
    let colorings = nested_to_array2(colorings);
    let sizes = Array1::from(sizes);

    (colorings, sizes, legal_moves)
}
