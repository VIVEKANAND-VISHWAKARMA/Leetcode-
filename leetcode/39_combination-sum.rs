// 39. Combination Sum
// Approach: Sort candidates and use backtracking to explore combinations.
// At each step, either reuse the current candidate or advance to the next,
// pruning branches early when the candidate exceeds the remaining target.
// Time Complexity: O(N^(T/M)) where N is candidates.len(), T is target, M is min candidate.
// Space Complexity: O(T/M) recursion stack and path storage.

struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut current, &mut result);
        result
    }

    fn backtrack(
        candidates: &[i32],
        remain: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remain == 0 {
            result.push(current.clone());
            return;
        }

        for i in start..candidates.len() {
            let val = candidates[i];
            if val > remain {
                break;
            }
            current.push(val);
            Self::backtrack(candidates, remain - val, i, current, result);
            current.pop();
        }
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 3, 6, 7], 7),
        (vec![2, 3, 5], 8),
        (vec![2], 1),
    ];

    for (candidates, target) in test_cases {
        println!("Input: candidates = {:?}, target = {}", candidates, target);
        let output = Solution::combination_sum(candidates, target);
        println!("Output: {:?}\n", output);
    }
}
