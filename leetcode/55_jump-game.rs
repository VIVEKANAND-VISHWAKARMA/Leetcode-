// 55. Jump Game
// Approach: Greedily track the maximum reachable index while iterating through the array.
// If the current index exceeds the maximum reachable index, the end cannot be reached.
// Time Complexity: O(n) where n is the length of nums.
// Space Complexity: O(1) auxiliary space.

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for (i, &jump) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            max_reach = max(max_reach, i + jump as usize);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

fn main() {
    let test_cases = vec![
        vec![2, 3, 1, 1, 4],
        vec![3, 2, 1, 0, 4],
    ];

    for nums in test_cases {
        let result = Solution::can_jump(nums.clone());
        println!("Input: nums = {:?}\nOutput: {}\n", nums, result);
    }
}
