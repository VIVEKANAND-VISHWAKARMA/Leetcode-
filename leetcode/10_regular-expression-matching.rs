/*
Approach: 2D Dynamic Programming where dp[i][j] indicates if s[0..i] matches p[0..j].
We handle '*' by either ignoring the pattern pair (dp[i][j-2]) or consuming a matching
character from s (dp[i-1][j]). Normal characters and '.' update from dp[i-1][j-1].
Time Complexity: O(m * n) where m = s.len(), n = p.len().
Space Complexity: O(m * n) for the DP table.
*/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let m = s_bytes.len();
        let n = p_bytes.len();

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for j in 2..=n {
            if p_bytes[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p_bytes[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2];
                    let prev_p = p_bytes[j - 2];
                    if prev_p == b'.' || prev_p == s_bytes[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else if p_bytes[j - 1] == b'.' || p_bytes[j - 1] == s_bytes[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n]
    }
}

fn main() {
    let test_cases = vec![
        ("aa", "a"),
        ("aa", "a*"),
        ("ab", ".*"),
    ];

    for (s, p) in test_cases {
        let result = Solution::is_match(s.to_string(), p.to_string());
        println!("s = \"{}\", p = \"{}\" => {}", s, p, result);
    }
}
