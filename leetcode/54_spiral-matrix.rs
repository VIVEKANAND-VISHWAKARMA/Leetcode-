// 54. Spiral Matrix
// Maintain four boundaries (top, bottom, left, right) and shrink them inward after traversing each edge.
// Traverse right along the top, down along the right, left along the bottom, and up along the left.
// Time Complexity: O(m * n) since every element is visited exactly once.
// Space Complexity: O(1) auxiliary space, ignoring the output vector.

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.is_empty() || matrix[0].is_empty() {
            return result;
        }

        let mut top = 0i32;
        let mut bottom = matrix.len() as i32 - 1;
        let mut left = 0i32;
        let mut right = matrix[0].len() as i32 - 1;

        while top <= bottom && left <= right {
            for col in left..=right {
                result.push(matrix[top as usize][col as usize]);
            }
            top += 1;

            for row in top..=bottom {
                result.push(matrix[row as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for col in (left..=right).rev() {
                    result.push(matrix[bottom as usize][col as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for row in (top..=bottom).rev() {
                    result.push(matrix[row as usize][left as usize]);
                }
                left += 1;
            }
        }

        result
    }
}

fn main() {
    let example1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    println!("Input: {:?}", example1);
    println!("Output: {:?}", Solution::spiral_order(example1));

    let example2 = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    println!("Input: {:?}", example2);
    println!("Output: {:?}", Solution::spiral_order(example2));
}
