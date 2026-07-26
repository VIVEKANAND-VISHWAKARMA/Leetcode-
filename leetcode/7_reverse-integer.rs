// Approach: Iteratively extract the last digit using modulo and append it to the result.
// Overflow is safely handled without 64-bit integers by using `checked_mul` and `checked_add`.
// Time Complexity: O(log10(|x|)), which is O(1) bounded by 32-bit integer limits.
// Space Complexity: O(1).

struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            match rev.checked_mul(10).and_then(|r| r.checked_add(pop)) {
                Some(next_rev) => rev = next_rev,
                None => return 0,
            }
        }
        rev
    }
}

fn main() {
    let test_cases = vec![123, -123, 120];

    for x in test_cases {
        let result = Solution::reverse(x);
        println!("Input: x = {}\nOutput: {}\n", x, result);
    }
}
