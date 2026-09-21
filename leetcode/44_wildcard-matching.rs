// 44. Wildcard Matching
// Uses a two-pointer greedy backtracking approach: when a '*' is encountered, record its position
// and the current string index. On mismatch, backtrack to the last '*' and let it consume one more
// character. Finally, check if remaining pattern characters are all '*'.
// Time complexity: O(s.len() * p.len()) worst-case, O(s.len() + p.len()) average.
// Space complexity: O(1) auxiliary space beyond string bytes.

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut s_idx = 0;
        let mut p_idx = 0;
        let mut star_idx: Option<usize> = None;
        let mut s_temp_idx = 0;

        while s_idx < s.len() {
            if p_idx < p.len() && (p[p_idx] == b'?' || p[p_idx] == s[s_idx]) {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p.len() && p[p_idx] == b'*' {
                star_idx = Some(p_idx);
                p_idx += 1;
                s_temp_idx = s_idx;
            } else if let Some(st) = star_idx {
                p_idx = st + 1;
                s_temp_idx += 1;
                s_idx = s_temp_idx;
            } else {
                return false;
            }
        }

        while p_idx < p.len() && p[p_idx] == b'*' {
            p_idx += 1;
        }

        p_idx == p.len()
    }
}

fn main() {
    let test_cases = [
        ("aa", "a"),
        ("aa", "*"),
        ("cb", "?a"),
    ];

    for (s, p) in test_cases {
        let result = Solution::is_match(s.to_string(), p.to_string());
        println!("s = \"{}\", p = \"{}\" -> {}", s, p, result);
    }
}
