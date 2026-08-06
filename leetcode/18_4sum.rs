// Approach: Sort the array and use two outer loops with a two-pointer search for the inner pair.
// Pruning checks and duplicate skipping avoid unnecessary work and ensure unique quadruplets.
// Time Complexity: O(n^3) where n is the length of nums.
// Space Complexity: O(1) auxiliary space (excluding the output vector).

struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = nums.len();
        if n < 4 {
            return result;
        }

        nums.sort_unstable();
        let target = target as i64;

        for i in 0..n.saturating_sub(3) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let val_i = nums[i] as i64;
            if val_i + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64 > target {
                break;
            }
            if val_i + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64 < target {
                continue;
            }

            for j in (i + 1)..n.saturating_sub(2) {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let val_j = nums[j] as i64;
                if val_i + val_j + nums[j + 1] as i64 + nums[j + 2] as i64 > target {
                    break;
                }
                if val_i + val_j + nums[n - 2] as i64 + nums[n - 1] as i64 < target {
                    continue;
                }

                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = val_i + val_j + nums[left] as i64 + nums[right] as i64;
                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let nums1 = vec![1, 0, -1, 0, -2, 2];
    let target1 = 0;
    println!("Input: nums = {:?}, target = {}", nums1, target1);
    let res1 = Solution::four_sum(nums1, target1);
    println!("Output: {:?}\n", res1);

    let nums2 = vec![2, 2, 2, 2, 2];
    let target2 = 8;
    println!("Input: nums = {:?}, target = {}", nums2, target2);
    let res2 = Solution::four_sum(nums2, target2);
    println!("Output: {:?}", res2);
}
