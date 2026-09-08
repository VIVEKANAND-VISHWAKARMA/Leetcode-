// 35. Search Insert Position
// Approach: Binary search for the lower bound (first position where nums[mid] >= target).
// Maintain a search space [left, right), adjusting left when nums[mid] < target and right otherwise.
// Time Complexity: O(log n)
// Space Complexity: O(1)

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 3, 5, 6], 5),
        (vec![1, 3, 5, 6], 2),
        (vec![1, 3, 5, 6], 7),
    ];

    for (nums, target) in test_cases {
        let result = Solution::search_insert(nums.clone(), target);
        println!("nums = {:?}, target = {} => output: {}", nums, target, result);
    }
}
