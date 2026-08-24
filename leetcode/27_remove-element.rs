// 27. Remove Element
// We use a two-pointer approach where a write pointer `k` tracks the position for the next valid element.
// We iterate through the array with a read pointer, copying elements not equal to `val` to index `k`.
// Time Complexity: O(n) to iterate through the array of length n.
// Space Complexity: O(1) auxiliary space as modifications are done in-place.

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    // Example 1
    let mut nums1 = vec![3, 2, 2, 3];
    let val1 = 3;
    println!("Example 1 Input: nums = {:?}, val = {}", nums1, val1);
    let k1 = Solution::remove_element(&mut nums1, val1);
    println!("Output: k = {}, nums = {:?}", k1, &nums1[..k1 as usize]);

    // Example 2
    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val2 = 2;
    println!("\nExample 2 Input: nums = {:?}, val = {}", nums2, val2);
    let k2 = Solution::remove_element(&mut nums2, val2);
    println!("Output: k = {}, nums = {:?}", k2, &nums2[..k2 as usize]);
}
