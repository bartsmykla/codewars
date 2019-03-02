#![allow(unused)]

/*
    Kata (4kyu): Validate Sudoku with size `NxN`
    Url: https://www.codewars.com/kata/validate-sudoku-with-size-nxn

    Given a Sudoku data structure with size NxN, N > 0 and √N == integer,
    write a method to validate if it has been filled out correctly.

    The data structure is a multi-dimensional Array(in Rust: Vec<Vec<u32>>), ie:

    [
      [7,8,4,  1,5,9,  3,2,6],
      [5,3,9,  6,7,2,  8,4,1],
      [6,1,2,  4,3,8,  7,5,9],

      [9,2,8,  7,1,5,  4,6,3],
      [3,5,7,  8,4,6,  1,9,2],
      [4,6,1,  9,2,3,  5,8,7],

      [8,7,6,  3,9,4,  2,1,5],
      [2,4,3,  5,6,1,  9,7,8],
      [1,9,5,  2,8,7,  6,3,4]
    ]

    Rules for validation

    Data structure dimension: NxN where N > 0 and √N == integer
    Rows may only contain integers: 1..N (N included)
    Columns may only contain integers: 1..N (N included)
    'Little squares' (3x3 in example above) may also only contain integers:
    1..N (N included)

    Note: the matrix may include non-integer elements.
*/

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let len = self.data.len();
        let len_f64 = len as f64;
        let len_triangle_number = ((len * len + len) / 2) as u32;

        if self.data.iter().any(|row| row.len() != len) {
            return false;
        }

        let mut rows = vec![0; len];
        let mut columns = vec![0; len];
        let mut squares = vec![0; len];

        for (row_i, row) in self.data.iter().enumerate() {
            for (col_i, &n) in row.iter().enumerate() {
                rows[row_i] += n;
                columns[col_i] += n;

                let row_square_n = (col_i as f64 / len_f64.sqrt()) as usize;
                let col_square_n =
                    (row_i as f64 / len_f64.sqrt()) as usize * len_f64.sqrt() as usize;

                squares[row_square_n + col_square_n] += n;
            }
        }

        !rows
            .iter()
            .chain(columns.iter())
            .chain(squares.iter())
            .any(|&i| i != len_triangle_number)
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
