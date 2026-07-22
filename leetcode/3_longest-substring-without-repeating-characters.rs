// Approach: Use a sliding window with a HashMap storing the last seen index of each character.
// If a character recurs within the current window, shift the left boundary past its last position.
// Time Complexity: O(N) where N is the number of characters in the string.
// Space Complexity: O(min(N, M)) where M is the size of the character set.

use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = HashMap::new();
        let mut max_len = 0;
        let mut left = 0;

        for (right, ch) in s.chars().enumerate() {
            if let Some(&prev_pos) = last_seen.get(&ch) {
                if prev_pos >= left {
                    left = prev_pos + 1;
                }
            }
            last_seen.insert(ch, right);
            max_len = max(max_len, right - left + 1);
        }

        max_len as i32
    }
}

fn main() {
    let test_cases = vec![
        "abcabcbb",
        "bbbbb",
        "pwwkew",
    ];

    for s in test_cases {
        let result = Solution::length_of_longest_substring(s.to_string());
        println!("Input: s = \"{}\"", s);
        println!("Output: {}", result);
    }
}
