// 28. Find the Index of the First Occurrence in a String
// Approach: Utilize Rust's standard string search `find()`, which implements
// the Two-Way string-matching algorithm to locate the needle's first occurrence.
// Time complexity: O(N + M) where N = haystack.len(), M = needle.len().
// Space complexity: O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
    }
}

fn main() {
    // Example 1
    let haystack1 = "sadbutsad".to_string();
    let needle1 = "sad".to_string();
    let result1 = Solution::str_str(haystack1.clone(), needle1.clone());
    println!("haystack: \"{}\", needle: \"{}\" => {}", haystack1, needle1, result1);

    // Example 2
    let haystack2 = "leetcode".to_string();
    let needle2 = "leeto".to_string();
    let result2 = Solution::str_str(haystack2.clone(), needle2.clone());
    println!("haystack: \"{}\", needle: \"{}\" => {}", haystack2, needle2, result2);
}
