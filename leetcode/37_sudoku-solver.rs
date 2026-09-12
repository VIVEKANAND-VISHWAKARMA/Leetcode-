// 37. Sudoku Solver
// Approach: Backtracking using bitmasks to track digit placement in rows, columns, and 3x3 boxes.
// Employs the Minimum Remaining Values (MRV) heuristic to prioritize the empty cell with the fewest candidates.
// Time Complexity: O(9^m) worst-case where m <= 81 is the number of empty cells; practically near-instant.
// Space Complexity: O(m) recursion stack depth, bounded by 81 -> O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let d = board[r][c].to_digit(10).unwrap() as usize;
                    let mask = 1 << d;
                    rows[r] |= mask;
                    cols[c] |= mask;
                    boxes[(r / 3) * 3 + (c / 3)] |= mask;
                }
            }
        }

        Self::backtrack(board, &mut rows, &mut cols, &mut boxes);
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
    ) -> bool {
        let mut min_candidates = 10;
        let mut best_cell = None;
        let mut best_mask = 0u16;

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    let b = (r / 3) * 3 + (c / 3);
                    let used = rows[r] | cols[c] | boxes[b];
                    let candidates = (!used) & 0x3FE; // Bits 1 through 9
                    let count = candidates.count_ones();

                    if count == 0 {
                        return false; // Dead end
                    }

                    if count < min_candidates {
                        min_candidates = count;
                        best_cell = Some((r, c, b));
                        best_mask = candidates;
                        if count == 1 {
                            break;
                        }
                    }
                }
            }
            if min_candidates == 1 {
                break;
            }
        }

        let (r, c, b) = match best_cell {
            Some(cell) => cell,
            None => return true, // All cells filled successfully
        };

        let mut mask = best_mask;
        while mask != 0 {
            let lowest_bit = mask & (!mask + 1);
            let d = lowest_bit.trailing_zeros() as u8;

            board[r][c] = (b'0' + d) as char;
            rows[r] |= lowest_bit;
            cols[c] |= lowest_bit;
            boxes[b] |= lowest_bit;

            if Self::backtrack(board, rows, cols, boxes) {
                return true;
            }

            rows[r] &= !lowest_bit;
            cols[c] &= !lowest_bit;
            boxes[b] &= !lowest_bit;
            board[r][c] = '.';

            mask &= mask - 1;
        }

        false
    }
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("Input Board:");
    for row in &board {
        println!("{:?}", row);
    }

    Solution::solve_sudoku(&mut board);

    println!("\nSolved Board:");
    for row in &board {
        println!("{:?}", row);
    }
}
