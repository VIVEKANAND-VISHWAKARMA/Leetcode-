// 49. Group Anagrams
// Approach: Count character frequencies for each string into a 26-element array [u8; 26]
// and use it as a key in a HashMap to group anagrams together.
// Time Complexity: O(N * K) where N is the number of strings and K is the maximum string length.
// Space Complexity: O(N * K) to store the grouped strings in the hash map.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut count = [0u8; 26];
            for &b in s.as_bytes() {
                count[(b - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }

        groups.into_values().collect()
    }
}

fn main() {
    // Example 1
    let strs1 = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!("Input: strs = {:?}", strs1);
    let result1 = Solution::group_anagrams(strs1);
    println!("Output: {:?}\n", result1);

    // Example 2
    let strs2 = vec!["".to_string()];
    println!("Input: strs = {:?}", strs2);
    let result2 = Solution::group_anagrams(strs2);
    println!("Output: {:?}\n", result2);

    // Example 3
    let strs3 = vec!["a".to_string()];
    println!("Input: strs = {:?}", strs3);
    let result3 = Solution::group_anagrams(strs3);
    println!("Output: {:?}", result3);
}
