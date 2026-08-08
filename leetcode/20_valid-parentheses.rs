// Approach: Use a stack to track expected matching closing brackets.
// For every opening bracket, push its matching closing bracket; for a closing bracket, pop and compare.
// Time Complexity: O(n) where n is the length of string s.
// Space Complexity: O(n) for the stack in the worst-case scenario.

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let examples = vec!["()", "()[]{}", "(]", "([])", "([)]"];

    for s in examples {
        let result = Solution::is_valid(s.to_string());
        println!("Input: s = \"{}\" | Output: {}", s, result);
    }
}
