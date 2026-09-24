use pagetop::prelude::*;

// Las pistas de columna y de fila se guardan aparte para que `Grid` sea pequeño; esta prueba
// avisa si algo vuelve a inflarlo.
#[pagetop::test]
async fn grid_stays_small() {
    assert!(
        std::mem::size_of::<Grid>() <= 1024,
        "Grid ocupa {} bytes",
        std::mem::size_of::<Grid>()
    );
}

#[pagetop::test]
async fn getters_return_the_default_until_tracks_are_set() {
    let grid = Grid::new();
    assert_eq!(grid.columns(), Responsive::default());
    assert_eq!(grid.rows(), Responsive::default());

    let tracks = grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0));
    let grid = grid.with_columns(tracks);
    assert_eq!(grid.columns(), Responsive::default().set(tracks));
    // Las filas siguen sin definir.
    assert_eq!(grid.rows(), Responsive::default());
}

// Un clon comparte las pistas hasta que se modifica; entonces cada uno tiene las suyas.
#[pagetop::test]
async fn a_clone_does_not_share_later_changes() {
    let three = grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0));
    let two = grid::Tracks::repeat(2, grid::AxisTrack::Fraction(1.0));
    let original = Grid::new().with_columns(three);
    let copy = original.clone().with_columns(two).with_rows(two);

    assert_eq!(original.columns(), Responsive::default().set(three));
    assert_eq!(original.rows(), Responsive::default());
    assert_eq!(copy.columns(), Responsive::default().set(two));
    assert_eq!(copy.rows(), Responsive::default().set(two));
}

#[pagetop::test]
async fn columns_and_rows_by_breakpoint_are_kept() {
    let one = grid::Tracks::repeat(1, grid::AxisTrack::Fraction(1.0));
    let four = grid::Tracks::repeat(4, grid::AxisTrack::Fraction(1.0));
    let grid = Grid::new()
        .with_columns(one)
        .with_columns_at(Breakpoint::Lg, four)
        .with_rows_at(Breakpoint::Md, one);
    assert_eq!(
        grid.columns(),
        Responsive::default().set(one).set_at(Breakpoint::Lg, four)
    );
    assert_eq!(
        grid.rows(),
        Responsive::default().set_at(Breakpoint::Md, one)
    );
}
