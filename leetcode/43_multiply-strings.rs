// 43. Multiply Strings
// Simulates grade-school column multiplication into a result array of size m + n.
// For each digit pair at indices i and j, add the product to position i + j + 1 and propagate carry to i + j.
// Leading zeros are skipped to form the final string, returning "0" if the result is zero.
// Time Complexity: O(m * n), Space Complexity: O(m + n) where m and n are the lengths of num1 and num2.

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();
        let len1 = n1.len();
        let len2 = n2.len();
        let mut result = vec![0u32; len1 + len2];

        for i in (0..len1).rev() {
            for j in (0..len2).rev() {
                let mul = (n1[i] - b'0') as u32 * (n2[j] - b'0') as u32;
                let sum = mul + result[i + j + 1];

                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        let start = result.iter().position(|&x| x != 0).unwrap_or(result.len());
        result[start..]
            .iter()
            .map(|&digit| (digit as u8 + b'0') as char)
            .collect()
    }
}

fn main() {
    let test_cases = [
        ("2", "3"),
        ("123", "456"),
    ];

    for (num1, num2) in test_cases {
        let result = Solution::multiply(num1.to_string(), num2.to_string());
        println!("num1 = \"{}\", num2 = \"{}\" => \"{}\"", num1, num2, result);
    }
}
