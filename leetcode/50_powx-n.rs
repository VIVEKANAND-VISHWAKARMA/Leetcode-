// 50. Pow(x, n)
// Uses binary exponentiation (exponentiation by squaring) while casting n to i64
// to prevent integer overflow when negating i32::MIN.
// Time Complexity: O(log |n|) as the exponent is halved in each iteration.
// Space Complexity: O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut exp = n as i64;
        let mut base = x;

        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }

        let mut result = 1.0;
        let mut current_product = base;

        while exp > 0 {
            if exp % 2 == 1 {
                result *= current_product;
            }
            current_product *= current_product;
            exp /= 2;
        }

        result
    }
}

fn main() {
    let test_cases = vec![
        (2.00000, 10),
        (2.10000, 3),
        (2.00000, -2),
    ];

    for (x, n) in test_cases {
        let result = Solution::my_pow(x, n);
        println!("x = {:.5}, n = {} => {:.5}", x, n, result);
    }
}
