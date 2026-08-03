// Approach: Sort the array and iterate with a fixed element, using two pointers for the rest.
// Skip identical adjacent elements at all three pointer positions to prevent duplicate triplets.
// Time Complexity: O(N^2) where N is the length of `nums`.
// Space Complexity: O(1) auxiliary space (excluding the output array).

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = nums.len();
        if len < 3 {
            return result;
        }

        nums.sort_unstable();

        for i in 0..len - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

fn main() {
    let test_cases = vec![
        vec![-1, 0, 1, 2, -1, -4],
        vec![0, 1, 1],
        vec![0, 0, 0],
    ];

    for nums in test_cases {
        let res = Solution::three_sum(nums.clone());
        println!("Input: nums = {:?}\nOutput: {:?}\n", nums, res);
    }
}
