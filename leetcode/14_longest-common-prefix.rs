// Approach: Perform vertical scanning by comparing characters at each index across all strings.
// Stop at the first mismatch or when reaching the end of any string.
// Time Complexity: O(S), where S is the total number of characters in all strings.
// Space Complexity: O(1) auxiliary space (excluding the output string).

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let first = strs[0].as_bytes();
        for i in 0..first.len() {
            let byte = first[i];
            for s in strs.iter().skip(1) {
                let s_bytes = s.as_bytes();
                if i >= s_bytes.len() || s_bytes[i] != byte {
                    return strs[0][..i].to_string();
                }
            }
        }
        strs[0].clone()
    }
}

fn main() {
    let strs1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Input: {:?}", strs1);
    println!("Output: \"{}\"", Solution::longest_common_prefix(strs1));

    let strs2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    println!("Input: {:?}", strs2);
    println!("Output: \"{}\"", Solution::longest_common_prefix(strs2));
}
