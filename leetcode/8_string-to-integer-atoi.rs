// Approach: Skip leading whitespace, process optional sign, and convert digit characters.
// Use i64 arithmetic to detect and handle 32-bit signed integer overflow/underflow.
// Time Complexity: O(N) where N is the length of the string s.
// Space Complexity: O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut i = 0;

        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        if i == n {
            return 0;
        }

        let mut sign = 1;
        if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        } else if bytes[i] == b'+' {
            i += 1;
        }

        let mut res: i64 = 0;
        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i64;
            res = res * 10 + digit;

            if sign == 1 && res >= i32::MAX as i64 {
                return i32::MAX;
            }
            if sign == -1 && -res <= i32::MIN as i64 {
                return i32::MIN;
            }

            i += 1;
        }

        (res * sign) as i32
    }
}

fn main() {
    let test_cases = vec![
        "42",
        "   -042",
        "1337c0d3",
        "0-1",
        "words and 987",
    ];

    for s in test_cases {
        let result = Solution::my_atoi(s.to_string());
        println!("Input: s = \"{}\"", s);
        println!("Output: {}\n", result);
    }
}
