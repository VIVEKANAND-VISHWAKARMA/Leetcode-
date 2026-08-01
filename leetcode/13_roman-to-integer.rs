// Approach: Traverse the string left to right, comparing each symbol's value with the next.
// If current < next, subtract current value from total; otherwise, add it.
// Time Complexity: O(N) where N is the length of the input string.
// Space Complexity: O(1) auxiliary space used.

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut total = 0;
        let len = bytes.len();

        for i in 0..len {
            let curr = match bytes[i] {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            };

            if i + 1 < len {
                let next = match bytes[i + 1] {
                    b'I' => 1,
                    b'V' => 5,
                    b'X' => 10,
                    b'L' => 50,
                    b'C' => 100,
                    b'D' => 500,
                    b'M' => 1000,
                    _ => 0,
                };
                if curr < next {
                    total -= curr;
                } else {
                    total += curr;
                }
            } else {
                total += curr;
            }
        }

        total
    }
}

fn main() {
    let test_cases = vec!["III", "LVIII", "MCMXCIV"];

    for s in test_cases {
        let result = Solution::roman_to_int(s.to_string());
        println!("Input: s = \"{}\", Output: {}", s, result);
    }
}
