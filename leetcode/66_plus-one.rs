// 66. Plus One
// Traverse the array from right to left; increment the first non-9 digit and return immediately.
// If a digit is 9, set it to 0 and continue propagating the carry.
// If all digits were 9, prepend a 1 to the vector.
// Time Complexity: O(n) where n is the number of digits.
// Space Complexity: O(1) auxiliary space (O(n) if reallocating when all digits are 9).

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        digits.insert(0, 1);
        digits
    }
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 3],
        vec![4, 3, 2, 1],
        vec![9],
    ];

    for digits in test_cases {
        let input = digits.clone();
        let result = Solution::plus_one(digits);
        println!("Input: digits = {:?} => Output: {:?}", input, result);
    }
}
