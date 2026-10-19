// 64. Minimum Path Sum
// Approach: Dynamic programming with a 1D array representing the minimum path sums along each column.
// For each cell (i, j), dp[j] = grid[i][j] + min(dp[j] (from top), dp[j-1] (from left)).
// Time complexity: O(m * n) where m and n are grid dimensions.
// Space complexity: O(n) using a single row DP buffer.

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![0; n];

        dp[0] = grid[0][0];
        for j in 1..n {
            dp[j] = dp[j - 1] + grid[0][j];
        }

        for i in 1..m {
            dp[0] += grid[i][0];
            for j in 1..n {
                dp[j] = grid[i][j] + dp[j].min(dp[j - 1]);
            }
        }

        dp[n - 1]
    }
}

fn main() {
    let example1 = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!("Input: grid = {:?}", example1);
    let result1 = Solution::min_path_sum(example1);
    println!("Output: {}", result1);

    let example2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("Input: grid = {:?}", example2);
    let result2 = Solution::min_path_sum(example2);
    println!("Output: {}", result2);
}
