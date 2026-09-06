// 33. Search in Rotated Sorted Array
// Perform a modified binary search by identifying which half of the current range is sorted.
// If the target falls within the boundaries of the sorted half, narrow the search to it;
// otherwise, search the opposite half.
// Time Complexity: O(log n), Space Complexity: O(1).

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid;
            }

            let left_val = nums[left as usize];
            let right_val = nums[right as usize];

            // Left half is sorted
            if left_val <= mid_val {
                if left_val <= target && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // Right half is sorted
                if mid_val < target && target <= right_val {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}

fn main() {
    let test_cases = vec![
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 3),
        (vec![1], 0),
    ];

    for (nums, target) in test_cases {
        let result = Solution::search(nums.clone(), target);
        println!("nums = {:?}, target = {} => {}", nums, target, result);
    }
}
