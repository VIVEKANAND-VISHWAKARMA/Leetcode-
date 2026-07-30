// Two-pointer approach: Start at both ends and compute the area bounded by the shorter line.
// Move the pointer corresponding to the shorter height inward, attempting to find a taller line.
// Time Complexity: O(n) - single pass over the array.
// Space Complexity: O(1) - constant auxiliary space.

use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_water = 0;

        while left < right {
            let h_left = height[left];
            let h_right = height[right];
            let water = (right - left) as i32 * min(h_left, h_right);
            max_water = max(max_water, water);

            if h_left < h_right {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}

fn main() {
    let height1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Input: height = {:?}", height1);
    println!("Output: {}", Solution::max_area(height1));

    let height2 = vec![1, 1];
    println!("Input: height = {:?}", height2);
    println!("Output: {}", Solution::max_area(height2));
}
