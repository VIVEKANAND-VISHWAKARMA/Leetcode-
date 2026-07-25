// Approach: Simulate the zigzag pattern row by row using a vector of Strings.
// Iterate through the characters, tracking the current row and direction, then concatenate the rows.
// Time Complexity: O(N), where N is the length of the string.
// Space Complexity: O(N) to store characters in row buffers.

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut curr_row = 0;
        let mut going_down = false;

        for c in s.chars() {
            rows[curr_row].push(c);
            if curr_row == 0 || curr_row == num_rows - 1 {
                going_down = !going_down;
            }
            if going_down {
                curr_row += 1;
            } else {
                curr_row -= 1;
            }
        }

        rows.concat()
    }
}

fn main() {
    let test_cases = vec![
        ("PAYPALISHIRING", 3),
        ("PAYPALISHIRING", 4),
        ("A", 1),
    ];

    for (s, num_rows) in test_cases {
        let result = Solution::convert(s.to_string(), num_rows);
        println!("Input: s = \"{}\", numRows = {}", s, num_rows);
        println!("Output: \"{}\"\n", result);
    }
}
