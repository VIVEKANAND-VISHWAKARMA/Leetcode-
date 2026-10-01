// 51. N-Queens
// Uses depth-first backtracking row-by-row with bitmasks to track occupied columns and diagonals.
// Available positions in each row are quickly identified via bitwise operations.
// Time complexity: O(N!), where N is the board dimension.
// Space complexity: O(N) auxiliary space for recursion and current board state (excluding output).

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        Self::backtrack(0, n as usize, 0, 0, 0, &mut board, &mut results);
        results
    }

    fn backtrack(
        row: usize,
        n: usize,
        cols: i32,
        diag1: i32,
        diag2: i32,
        board: &mut Vec<Vec<char>>,
        results: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let solution = board
                .iter()
                .map(|r| r.iter().collect::<String>())
                .collect();
            results.push(solution);
            return;
        }

        let mut available = ((1 << n) - 1) & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & -available;
            available &= available - 1;
            let col = bit.trailing_zeros() as usize;

            board[row][col] = 'Q';
            Self::backtrack(
                row + 1,
                n,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                board,
                results,
            );
            board[row][col] = '.';
        }
    }
}

fn main() {
    let test_cases = vec![4, 1];

    for n in test_cases {
        println!("Input: n = {}", n);
        let result = Solution::solve_n_queens(n);
        println!("Output: [");
        for (i, solution) in result.iter().enumerate() {
            print!("  [");
            for (j, row) in solution.iter().enumerate() {
                if j > 0 {
                    print!(", ");
                }
                print!("\"{}\"", row);
            }
            if i + 1 < result.len() {
                println!("],");
            } else {
                println!("]");
            }
        }
        println!("]");
        println!();
    }
}
