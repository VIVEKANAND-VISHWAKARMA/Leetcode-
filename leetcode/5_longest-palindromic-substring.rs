/*
Approach: Expand around center for both odd and even length palindromes.
Iterate through all possible centers, expanding outwards while characters match.
Time Complexity: O(N^2), where N is the length of string s.
Space Complexity: O(1) auxiliary space (using byte slices for ASCII indexing).
*/

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        if bytes.len() <= 1 {
            return s;
        }

        let mut start = 0;
        let mut max_len = 0;

        for i in 0..bytes.len() {
            let (s1, l1) = Self::expand_around_center(bytes, i as i32, i as i32);
            if l1 > max_len {
                start = s1;
                max_len = l1;
            }

            let (s2, l2) = Self::expand_around_center(bytes, i as i32, i as i32 + 1);
            if l2 > max_len {
                start = s2;
                max_len = l2;
            }
        }

        s[start..start + max_len].to_string()
    }

    fn expand_around_center(bytes: &[u8], mut left: i32, mut right: i32) -> (usize, usize) {
        while left >= 0 && (right as usize) < bytes.len() && bytes[left as usize] == bytes[right as usize] {
            left -= 1;
            right += 1;
        }
        let start = (left + 1) as usize;
        let len = (right - left - 1) as usize;
        (start, len)
    }
}

fn main() {
    let test_cases = vec!["babad", "cbbd"];
    for s in test_cases {
        let result = Solution::longest_palindrome(s.to_string());
        println!("Input: s = \"{}\"", s);
        println!("Output: \"{}\"", result);
    }
}
