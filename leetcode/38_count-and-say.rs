// 38. Count and Say
// Approach: Iteratively generate the sequence starting from "1". For each step up to n,
// scan consecutive identical characters to compute run lengths and construct the next string.
// Time Complexity: O(L) where L is the total length of all intermediate strings, bounded by Conway's constant (~1.3035^n).
// Space Complexity: O(M) where M is the length of the nth string (~4462 chars for n = 30).

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut curr = String::from("1");

        for _ in 1..n {
            let mut next = String::new();
            let bytes = curr.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                let mut count = 1;
                while i + 1 < bytes.len() && bytes[i] == bytes[i + 1] {
                    count += 1;
                    i += 1;
                }
                next.push_str(&count.to_string());
                next.push(bytes[i] as char);
                i += 1;
            }

            curr = next;
        }

        curr
    }
}

fn main() {
    let test_cases = vec![1, 4];

    for n in test_cases {
        let result = Solution::count_and_say(n);
        println!("Input: n = {}", n);
        println!("Output: \"{}\"", result);
    }
}
