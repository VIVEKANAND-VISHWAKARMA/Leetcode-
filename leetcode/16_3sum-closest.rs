/*
Approach: Sort the array and use a fixed element combined with a two-pointer search to find the sum closest to target.
Time Complexity: O(n^2) where n is the length of nums.
Space Complexity: O(1) auxiliary space (excluding internal sorting space).
*/

struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut closest_sum = nums[0] + nums[1] + nums[2];

        for i in 0..n - 2 {
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }

                if sum == target {
                    return target;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        closest_sum
    }
}

fn main() {
    let nums1 = vec![-1, 2, 1, -4];
    let target1 = 1;
    let res1 = Solution::three_sum_closest(nums1.clone(), target1);
    println!("Input: nums = {:?}, target = {}", nums1, target1);
    println!("Output: {}", res1);

    let nums2 = vec![0, 0, 0];
    let target2 = 1;
    let res2 = Solution::three_sum_closest(nums2.clone(), target2);
    println!("Input: nums = {:?}, target = {}", nums2, target2);
    println!("Output: {}", res2);
}
