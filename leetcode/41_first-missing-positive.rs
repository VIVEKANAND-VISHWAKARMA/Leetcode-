// 41. First Missing Positive
// Uses cycle sort to place each number x in range [1, n] at index x - 1 via swaps.
// A second pass finds the first index i where nums[i] != i + 1, returning i + 1.
// If all indices match, the answer is n + 1.
// Time complexity: O(n) as each element is placed in its correct position at most once.
// Space complexity: O(1) auxiliary space by modifying the input array in place.

struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 {
                let target_idx = (nums[i] - 1) as usize;
                if nums[target_idx] == nums[i] {
                    break;
                }
                nums.swap(i, target_idx);
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 0],
        vec![3, 4, -1, 1],
        vec![7, 8, 9, 11, 12],
    ];

    for nums in test_cases {
        let input_clone = nums.clone();
        let result = Solution::first_missing_positive(nums);
        println!("Input: nums = {:?}, Output: {}", input_clone, result);
    }
}
