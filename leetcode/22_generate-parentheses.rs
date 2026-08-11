/*
 * Approach: Backtracking using recursive depth-first search.
 * Recursively append '(' when open < n and ')' when close < open to maintain validity.
 * Time Complexity: O(4^n / sqrt(n)), bounded by the n-th Catalan number.
 * Space Complexity: O(n) auxiliary space for recursion stack depth and temporary string.
 */

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut current = String::with_capacity(2 * n);
        Self::backtrack(n, 0, 0, &mut current, &mut result);
        result
    }

    fn backtrack(n: usize, open: usize, close: usize, current: &mut String, result: &mut Vec<String>) {
        if current.len() == 2 * n {
            result.push(current.clone());
            return;
        }

        if open < n {
            current.push('(');
            Self::backtrack(n, open + 1, close, current, result);
            current.pop();
        }

        if close < open {
            current.push(')');
            Self::backtrack(n, open, close + 1, current, result);
            current.pop();
        }
    }
}

fn main() {
    let test_cases = vec![3, 1];
    for n in test_cases {
        let result = Solution::generate_parenthesis(n);
        println!("Input: n = {}\nOutput: {:?}", n, result);
    }
}
