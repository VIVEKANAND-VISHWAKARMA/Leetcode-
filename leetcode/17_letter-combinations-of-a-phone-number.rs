/*
 * Approach: Backtracking / DFS to recursively generate all possible letter combinations.
 * Maps each digit '2'-'9' to its corresponding set of characters and iterates through them.
 * Time Complexity: O(4^N) where N is the length of digits (at most 4 letters per digit).
 * Space Complexity: O(N) auxiliary space for recursion call stack and prefix string construction.
 */

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mapping = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut result = Vec::new();
        let mut current = String::new();
        let digits_bytes = digits.as_bytes();

        fn backtrack(
            idx: usize,
            digits: &[u8],
            mapping: &[&str; 10],
            current: &mut String,
            result: &mut Vec<String>,
        ) {
            if idx == digits.len() {
                result.push(current.clone());
                return;
            }

            let digit_idx = (digits[idx] - b'0') as usize;
            for ch in mapping[digit_idx].chars() {
                current.push(ch);
                backtrack(idx + 1, digits, mapping, current, result);
                current.pop();
            }
        }

        backtrack(0, digits_bytes, &mapping, &mut current, &mut result);
        result
    }
}

fn main() {
    let test_cases = vec!["23", "2"];

    for digits in test_cases {
        let result = Solution::letter_combinations(digits.to_string());
        println!("Input: digits = \"{}\"", digits);
        println!("Output: {:?}", result);
    }
}
