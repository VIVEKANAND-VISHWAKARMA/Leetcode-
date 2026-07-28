// Reverses the second half of the integer and compares it to the first half.
// Negative numbers and numbers ending in 0 (except 0) are not palindromes.
// Time Complexity: O(log10(x))
// Space Complexity: O(1)

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x = x;
        let mut reversed_num = 0;

        while x > reversed_num {
            reversed_num = reversed_num * 10 + x % 10;
            x /= 10;
        }

        x == reversed_num || x == reversed_num / 10
    }
}

fn main() {
    let test_cases = vec![121, -121, 10];
    for x in test_cases {
        let result = Solution::is_palindrome(x);
        println!("Input: x = {}, Output: {}", x, result);
    }
}
