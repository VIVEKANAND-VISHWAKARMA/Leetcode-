// 36. Valid Sudoku
// Approach: Traverse the 9x9 board once, using bitmasks to track seen digits in each row,
// column, and 3x3 sub-box. For each filled cell, check if the digit bit is already set;
// if so, the board is invalid. Otherwise, set the bit and continue.
// Time Complexity: O(1) since the board size is fixed at 9x9 (81 cells).
// Space Complexity: O(1) using fixed-size arrays of bitmasks.

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue;
                }

                let val = (ch as u8 - b'1') as usize;
                let bit = 1u16 << val;
                let box_idx = (r / 3) * 3 + (c / 3);

                if (rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[box_idx] & bit) != 0 {
                    return false;
                }

                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box_idx] |= bit;
            }
        }

        true
    }
}

fn main() {
    let board1 = vec![
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

    println!("Example 1 Input: board =");
    for row in &board1 {
        println!("{:?}", row);
    }
    let res1 = Solution::is_valid_sudoku(board1);
    println!("Example 1 Output: {}\n", res1);

    let board2 = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("Example 2 Input: board =");
    for row in &board2 {
        println!("{:?}", row);
    }
    let res2 = Solution::is_valid_sudoku(board2);
    println!("Example 2 Output: {}", res2);
}
