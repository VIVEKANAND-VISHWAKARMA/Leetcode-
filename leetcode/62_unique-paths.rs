// 62. Unique Paths
// Computes the binomial coefficient C(m + n - 2, min(m - 1, n - 1)), representing
// the number of ways to choose down-steps out of total steps to reach the destination.
// Time complexity: O(min(m, n))
// Space complexity: O(1)

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let total_steps = (m + n - 2) as u128;
        let k = (m - 1).min(n - 1) as u128;
        let mut ans: u128 = 1;

        for i in 1..=k {
            ans = ans * (total_steps - k + i) / i;
        }

        ans as i32
    }
}

fn main() {
    let test_cases = [(3, 7), (3, 2)];

    for (m, n) in test_cases {
        let result = Solution::unique_paths(m, n);
        println!("Input: m = {}, n = {} => Output: {}", m, n, result);
    }
}
