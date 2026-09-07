// 34. Find First and Last Position of Element in Sorted Array
// Perform two binary searches using partition points: the first finds the earliest
// index where an element is >= target (lower bound), and the second finds the earliest
// index where an element is > target (upper bound). If the lower bound is out of range
// or its value differs from target, return [-1, -1]; otherwise, return [first, last - 1].
// Time Complexity: O(log n), Space Complexity: O(1).

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first = nums.partition_point(|&x| x < target);
        if first == nums.len() || nums[first] != target {
            return vec![-1, -1];
        }
        let last = nums.partition_point(|&x| x <= target);
        vec![first as i32, (last - 1) as i32]
    }
}

fn main() {
    let test_cases = vec![
        (vec![5, 7, 7, 8, 8, 10], 8),
        (vec![5, 7, 7, 8, 8, 10], 6),
        (vec![], 0),
    ];

    for (nums, target) in test_cases {
        let result = Solution::search_range(nums.clone(), target);
        println!("Input: nums = {:?}, target = {}", nums, target);
        println!("Output: {:?}\n", result);
    }
}
