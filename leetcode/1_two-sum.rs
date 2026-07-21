/*
 * One-pass Hash Map: Store visited numbers and their indices in a hash map.
 * Check if `target - num` exists in the map as we iterate.
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&prev_idx) = map.get(&complement) {
                return vec![prev_idx, i as i32];
            }
            map.insert(num, i as i32);
        }
        unreachable!()
    }
}
