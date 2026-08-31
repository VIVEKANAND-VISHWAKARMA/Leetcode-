// 29. Divide Two Integers
// Approach: Convert dividend and divisor to negative numbers to prevent overflow, then subtract
// exponentially doubled multiples of the divisor using bit shifts (addition/doubling).
// Time Complexity: O(log^2 N) or O(log N) where N is the magnitude of the dividend.
// Space Complexity: O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let is_negative = (dividend < 0) ^ (divisor < 0);

        let mut a = if dividend > 0 { -dividend } else { dividend };
        let b = if divisor > 0 { -divisor } else { divisor };

        let mut quotient = 0i32;

        while a <= b {
            let mut value = b;
            let mut count = -1;

            while value >= (i32::MIN >> 1) && a <= value + value {
                value += value;
                count += count;
            }

            a -= value;
            quotient += count;
        }

        if is_negative {
            quotient
        } else {
            -quotient
        }
    }
}

fn main() {
    let test_cases = [
        (10, 3),
        (7, -3),
    ];

    for &(dividend, divisor) in &test_cases {
        let result = Solution::divide(dividend, divisor);
        println!("dividend: {}, divisor: {} => {}", dividend, divisor, result);
    }
}
