// 67. Add Binary
// Traverse both binary strings from right to left, simulating column-by-column addition with a carry.
// Push each resulting bit into a byte buffer, then reverse and convert it into the final string.
// Time complexity: O(max(N, M)) where N and M are the lengths of strings a and b.
// Space complexity: O(max(N, M)) to store the resultant binary string.

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = a_bytes.len() as isize - 1;
        let mut j = b_bytes.len() as isize - 1;
        let mut carry = 0;
        let mut result = Vec::with_capacity(a_bytes.len().max(b_bytes.len()) + 1);

        while i >= 0 || j >= 0 || carry != 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as i32;
                j -= 1;
            }
            result.push(b'0' + (sum % 2) as u8);
            carry = sum / 2;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let test_cases = [
        ("11", "1"),
        ("1010", "1011"),
    ];

    for (a, b) in test_cases {
        let result = Solution::add_binary(a.to_string(), b.to_string());
        println!("Input: a = \"{}\", b = \"{}\"", a, b);
        println!("Output: \"{}\"", result);
    }
}
