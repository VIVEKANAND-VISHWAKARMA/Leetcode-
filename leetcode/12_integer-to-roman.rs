// Approach: Greedy matching using standard and subtractive Roman numeral values in descending order.
// Time Complexity: O(1) as num is bounded between 1 and 3999.
// Space Complexity: O(1) auxiliary space, using a fixed mapping table and bounded string output.

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mappings = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        for &(value, symbol) in mappings.iter() {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }
        result
    }
}

fn main() {
    let test_cases = [3749, 58, 1994];
    for &num in &test_cases {
        let result = Solution::int_to_roman(num);
        println!("Input: {}, Output: \"{}\"", num, result);
    }
}
