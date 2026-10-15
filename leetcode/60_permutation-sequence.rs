// 60. Permutation Sequence
// We determine the permutation digit by digit using the factorial number system (factoradic).
// Convert k to 0-indexed, and for each position, find the index of the unused digit via
// k / (n - 1)!, append it, remove it from the pool, and update k = k % (n - 1)!.
// Time Complexity: O(n^2) due to shifting elements on removal (n <= 9, virtually O(1)).
// Space Complexity: O(n) to store available digits and the result string.

struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = (k - 1) as usize;

        let mut factorials = vec![1; n];
        for i in 1..n {
            factorials[i] = factorials[i - 1] * i;
        }

        let mut digits: Vec<char> = (1..=n)
            .map(|d| char::from_digit(d as u32, 10).unwrap())
            .collect();

        let mut result = String::with_capacity(n);
        for i in (0..n).rev() {
            let index = k / factorials[i];
            result.push(digits.remove(index));
            k %= factorials[i];
        }

        result
    }
}

fn main() {
    let test_cases = [
        (3, 3),
        (4, 9),
        (3, 1),
    ];

    for (n, k) in test_cases {
        let result = Solution::get_permutation(n, k);
        println!("Input: n = {}, k = {} -> Output: \"{}\"", n, k, result);
    }
}
