// 53. Maximum Subarray
// Uses Kadane's algorithm to compute the maximum subarray sum in a single pass.
// At each step, either extend the previous subarray or start fresh from the current element.
// Time Complexity: O(n) where n is the length of nums.
// Space Complexity: O(1) auxiliary space.

use std::cmp;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for &num in &nums[1..] {
            current_sum = cmp::max(num, current_sum + num);
            max_sum = cmp::max(max_sum, current_sum);
        }

        max_sum
    }
}

fn main() {
    let test_cases = vec![
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        vec![1],
        vec![5, 4, -1, 7, 8],
    ];

    for nums in test_cases {
        let result = Solution::max_sub_array(nums.clone());
        println!("Input: nums = {:?}\nOutput: {}\n", nums, result);
    }
}
