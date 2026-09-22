// 45. Jump Game II
// Use a greedy BFS-like approach tracking the current jump's reach (`current_end`)
// and the farthest reachable index (`farthest`) from the current window.
// When iterating up to `n - 2`, increment the jump count whenever reaching `current_end`.
// Time Complexity: O(n) where n is the length of nums.
// Space Complexity: O(1) auxiliary space.

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..n - 1 {
            farthest = max(farthest, i + nums[i] as usize);
            if i == current_end {
                jumps += 1;
                current_end = farthest;
                if current_end >= n - 1 {
                    break;
                }
            }
        }

        jumps
    }
}

fn main() {
    let test_cases = vec![
        vec![2, 3, 1, 1, 4],
        vec![2, 3, 0, 1, 4],
    ];

    for nums in test_cases {
        let result = Solution::jump(nums.clone());
        println!("Input: nums = {:?}\nOutput: {}\n", nums, result);
    }
}
