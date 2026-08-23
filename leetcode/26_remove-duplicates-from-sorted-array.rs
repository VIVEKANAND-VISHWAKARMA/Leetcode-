// 26. Remove Duplicates from Sorted Array

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut insert_idx = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[insert_idx] = nums[i];
                insert_idx += 1;
            }
        }

        insert_idx as i32
    }
}

fn main() {
    let mut nums1 = vec![1, 1, 2];
    println!("Input: nums = {:?}", nums1);
    let k1 = Solution::remove_duplicates(&mut nums1);
    println!("Output: {}, nums = {:?}", k1, &nums1[..k1 as usize]);

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("Input: nums = {:?}", nums2);
    let k2 = Solution::remove_duplicates(&mut nums2);
    println!("Output: {}, nums = {:?}", k2, &nums2[..k2 as usize]);
}
