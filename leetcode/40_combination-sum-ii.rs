// 40. Combination Sum II
// Sort candidates to enable early stopping and to skip duplicate elements at the same recursion depth.
// Use backtracking to explore valid combinations, recursing with the next index since each element is used once.
// Time complexity: O(2^n) in the worst case, pruned significantly by sorting and target bounds.
// Space complexity: O(n) auxiliary space for the recursion stack and current combination path.

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut results = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut current, &mut results);
        results
    }

    fn backtrack(
        candidates: &[i32],
        remain: i32,
        start: usize,
        current: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if remain == 0 {
            results.push(current.clone());
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            if candidates[i] > remain {
                break;
            }
            current.push(candidates[i]);
            Self::backtrack(candidates, remain - candidates[i], i + 1, current, results);
            current.pop();
        }
    }
}

fn main() {
    let candidates1 = vec![10, 1, 2, 7, 6, 1, 5];
    let target1 = 8;
    println!("Input: candidates = {:?}, target = {}", candidates1, target1);
    let result1 = Solution::combination_sum2(candidates1, target1);
    println!("Output: {:?}", result1);

    let candidates2 = vec![2, 5, 2, 1, 2];
    let target2 = 5;
    println!("\nInput: candidates = {:?}, target = {}", candidates2, target2);
    let result2 = Solution::combination_sum2(candidates2, target2);
    println!("Output: {:?}", result2);
}
