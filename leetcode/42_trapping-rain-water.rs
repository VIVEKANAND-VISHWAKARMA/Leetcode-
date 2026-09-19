// 42. Trapping Rain Water
// Use a two-pointer approach moving inward from both ends, tracking the maximum height seen so far from the left and right.
// At each step, process the side with the smaller maximum since the trapped water at that position is bounded by that side's maximum.
// Time Complexity: O(n) single pass over the array.
// Space Complexity: O(1) auxiliary memory.

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut total_water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    total_water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    total_water += right_max - height[right];
                }
                right -= 1;
            }
        }

        total_water
    }
}

fn main() {
    let test_cases = vec![
        vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        vec![4, 2, 0, 3, 2, 5],
    ];

    for height in test_cases {
        let result = Solution::trap(height.clone());
        println!("Input: height = {:?}\nOutput: {}\n", height, result);
    }
}
