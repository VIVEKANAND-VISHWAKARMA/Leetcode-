// 58. Length of Last Word
// Approach: Traverse backwards from the end of the string, skipping any trailing spaces.
// Then count consecutive non-space characters until the next space or the start of the string.
// Time Complexity: O(n) where n is the length of the string.
// Space Complexity: O(1) auxiliary space.

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = bytes.len();
        let mut length = 0;

        while i > 0 && bytes[i - 1] == b' ' {
            i -= 1;
        }

        while i > 0 && bytes[i - 1] != b' ' {
            length += 1;
            i -= 1;
        }

        length
    }
}

fn main() {
    let test_cases = vec![
        "Hello World".to_string(),
        "   fly me   to   the moon  ".to_string(),
        "luffy is still joyboy".to_string(),
    ];

    for s in test_cases {
        let result = Solution::length_of_last_word(s.clone());
        println!("Input: s = \"{}\"", s);
        println!("Output: {}", result);
    }
}
