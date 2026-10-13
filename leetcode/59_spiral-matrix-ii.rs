// 59. Spiral Matrix II
// Approach: Maintain four boundaries (top, bottom, left, right) and fill
// the matrix layer by layer in clockwise direction until all n^2 cells are filled.
// Time Complexity: O(n^2) since every cell in the n x n matrix is visited once.
// Space Complexity: O(1) auxiliary space beyond the output matrix.

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n_usize = n as usize;
        let mut matrix = vec![vec![0; n_usize]; n_usize];
        let mut top = 0i32;
        let mut bottom = n - 1;
        let mut left = 0i32;
        let mut right = n - 1;
        let mut num = 1;
        let max_num = n * n;

        while num <= max_num {
            for col in left..=right {
                matrix[top as usize][col as usize] = num;
                num += 1;
            }
            top += 1;

            for row in top..=bottom {
                matrix[row as usize][right as usize] = num;
                num += 1;
            }
            right -= 1;

            if top <= bottom {
                for col in (left..=right).rev() {
                    matrix[bottom as usize][col as usize] = num;
                    num += 1;
                }
                bottom -= 1;
            }

            if left <= right {
                for row in (top..=bottom).rev() {
                    matrix[row as usize][left as usize] = num;
                    num += 1;
                }
                left += 1;
            }
        }

        matrix
    }
}

fn main() {
    let test_cases = vec![3, 1];

    for n in test_cases {
        let result = Solution::generate_matrix(n);
        println!("Input: n = {}", n);
        println!("Output: {:?}", result);
    }
}
