// 63. Unique Paths II
// Dynamic programming with 1D space optimization: maintain a vector representing
// paths to reach each column in the current row. For each cell, if it contains an
// obstacle, paths become 0; otherwise, add paths coming from the left cell.
// Time Complexity: O(m * n), Space Complexity: O(n).

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let mut dp = vec![0i32; n];

        dp[0] = 1;

        for row in obstacle_grid {
            for j in 0..n {
                if row[j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n - 1]
    }
}

fn main() {
    let example1 = vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0],
    ];
    let result1 = Solution::unique_paths_with_obstacles(example1.clone());
    println!("Input: obstacleGrid = {:?}", example1);
    println!("Output: {}", result1);

    let example2 = vec![
        vec![0, 1],
        vec![0, 0],
    ];
    let result2 = Solution::unique_paths_with_obstacles(example2.clone());
    println!("Input: obstacleGrid = {:?}", example2);
    println!("Output: {}", result2);
}
