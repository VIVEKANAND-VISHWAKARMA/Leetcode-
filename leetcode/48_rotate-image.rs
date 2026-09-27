// 48. Rotate Image
// To rotate clockwise by 90 degrees in-place, first transpose the matrix
// along its main diagonal (swap matrix[i][j] with matrix[j][i]), then
// reverse each row horizontally.
// Time Complexity: O(n^2) where n is the dimension of the matrix.
// Space Complexity: O(1) auxiliary space as all modifications are in-place.

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // Step 1: Transpose matrix
        for i in 0..n {
            for j in (i + 1)..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        // Step 2: Reverse each row
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

fn main() {
    let mut matrix1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    println!("Example 1 Input:  {:?}", matrix1);
    Solution::rotate(&mut matrix1);
    println!("Example 1 Output: {:?}", matrix1);

    let mut matrix2 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    println!("Example 2 Input:  {:?}", matrix2);
    Solution::rotate(&mut matrix2);
    println!("Example 2 Output: {:?}", matrix2);
}
