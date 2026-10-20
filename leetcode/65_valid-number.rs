// 65. Valid Number
// We track flags for seen digits, decimal dot, and exponent while traversing characters.
// Signs ('+' or '-') are valid only at the start or immediately following 'e' or 'E'.
// When 'e' or 'E' is seen, we reset the digit flag so that at least one digit must follow.
// Time complexity: O(n) where n is the length of s. Space complexity: O(1).

struct Solution;

impl Solution {
    pub fn is_valid_number(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut seen_digit = false;
        let mut seen_dot = false;
        let mut seen_exponent = false;

        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'0'..=b'9' => {
                    seen_digit = true;
                }
                b'+' | b'-' => {
                    if i > 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                        return false;
                    }
                }
                b'.' => {
                    if seen_dot || seen_exponent {
                        return false;
                    }
                    seen_dot = true;
                }
                b'e' | b'E' => {
                    if seen_exponent || !seen_digit {
                        return false;
                    }
                    seen_exponent = true;
                    seen_digit = false;
                }
                _ => return false,
            }
        }

        seen_digit
    }
}

fn main() {
    let test_cases = vec!["0", "e", "."];

    for s in test_cases {
        let result = Solution::is_valid_number(s.to_string());
        println!("Input: s = \"{}\", Output: {}", s, result);
    }
}
