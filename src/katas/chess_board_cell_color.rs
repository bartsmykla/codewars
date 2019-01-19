fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    cell1.as_bytes().iter().sum::<u8>() % 2 == cell2.as_bytes().iter().sum::<u8>() % 2
}

#[test]
fn basic_tests() {
    assert_eq!(chessboard_cell_color("A1", "C3"), true);
    assert_eq!(chessboard_cell_color("A1", "H3"), false);
    assert_eq!(chessboard_cell_color("A1", "A2"), false);
    assert_eq!(chessboard_cell_color("A1", "C1"), true);
    assert_eq!(chessboard_cell_color("A1", "A1"), true);
}