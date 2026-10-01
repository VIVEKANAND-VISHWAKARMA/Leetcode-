// 52. N-Queens II
// We use backtracking with bit manipulation to efficiently track available positions.
// Bitmasks represent occupied columns, main diagonals, and anti-diagonals.
// Time complexity: O(n!), as we prune invalid branches early.
// Space complexity: O(n) for the recursion stack.

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        Self::backtrack(0, 0, 0, 0, n, &mut count);
        count
    }

    fn backtrack(row: i32, cols: i32, diag1: i32, diag2: i32, n: i32, count: &mut i32) {
        if row == n {
            *count += 1;
            return;
        }

        let mut available = ((1 << n) - 1) & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & -available;
            available &= available - 1;
            Self::backtrack(
                row + 1,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                n,
                count,
            );
        }
    }
}

fn main() {
    let test_cases = vec![4, 1];

    for n in test_cases {
        let result = Solution::total_n_queens(n);
        println!("Input: n = {}\nOutput: {}\n", n, result);
    }
}
