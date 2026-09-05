// 32. Longest Valid Parentheses
// We use a two-pass counting approach: scanning left-to-right and right-to-left.
// Tracking '(' and ')' counts allows identifying valid substrings whenever counts match
// and resetting when invalid conditions occur, achieving maximal length.
// Time complexity: O(n), Space complexity: O(1).

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len = 0;
        let mut left = 0;
        let mut right = 0;

        for ch in s.chars() {
            if ch == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                max_len = max(max_len, 2 * right);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }

        left = 0;
        right = 0;

        for ch in s.chars().rev() {
            if ch == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                max_len = max(max_len, 2 * left);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }

        max_len
    }
}

fn main() {
    let test_cases = vec!["(()", ")()())", ""];

    for s in test_cases {
        let result = Solution::longest_valid_parentheses(s.to_string());
        println!("Input: s = \"{}\"", s);
        println!("Output: {}", result);
    }
}
