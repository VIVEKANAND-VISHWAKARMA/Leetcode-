// 68. Text Justification
// Approach: Greedily pack words per line ensuring length + single spaces <= maxWidth.
// For standard lines, distribute remaining spaces evenly among gaps, placing extra spaces on the left.
// For the last line or lines with a single word, left-justify with single spaces between words and pad right.
// Time Complexity: O(N) where N is the total number of characters in all words.
// Space Complexity: O(maxWidth) auxiliary space per line, O(total characters) for the result.

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut i = 0;
        let n = words.len();

        while i < n {
            let mut line_len = words[i].len();
            let mut j = i + 1;

            // Greedily find how many words can fit in this line
            while j < n && line_len + 1 + words[j].len() <= max_width {
                line_len += 1 + words[j].len();
                j += 1;
            }

            let num_words = j - i;
            let mut line = String::with_capacity(max_width);

            // Last line or single-word line: left-justify
            if j == n || num_words == 1 {
                for k in i..j {
                    if k > i {
                        line.push(' ');
                    }
                    line.push_str(&words[k]);
                }
                while line.len() < max_width {
                    line.push(' ');
                }
            } else {
                // Fully justify
                let total_chars: usize = words[i..j].iter().map(|w| w.len()).sum();
                let total_spaces = max_width - total_chars;
                let gaps = num_words - 1;
                let base_spaces = total_spaces / gaps;
                let extra_spaces = total_spaces % gaps;

                for k in i..j {
                    line.push_str(&words[k]);
                    if k < j - 1 {
                        let gap_idx = k - i;
                        let spaces_to_add = base_spaces + if gap_idx < extra_spaces { 1 } else { 0 };
                        for _ in 0..spaces_to_add {
                            line.push(' ');
                        }
                    }
                }
            }

            result.push(line);
            i = j;
        }

        result
    }
}

fn main() {
    let test_cases: Vec<(Vec<&str>, i32)> = vec![
        (
            vec!["This", "is", "an", "example", "of", "text", "justification."],
            16,
        ),
        (
            vec!["What", "must", "be", "acknowledgment", "shall", "be"],
            16,
        ),
        (
            vec![
                "Science", "is", "what", "we", "understand", "well", "enough", "to", "explain",
                "to", "a", "computer.", "Art", "is", "everything", "else", "we", "do",
            ],
            20,
        ),
    ];

    for (words, max_width) in test_cases {
        let words_string: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
        println!("Input: words = {:?}, maxWidth = {}", words, max_width);
        let output = Solution::full_justify(words_string, max_width);
        println!("Output:");
        println!("[");
        for line in &output {
            println!("  \"{}\"", line);
        }
        println!("]");
        println!();
    }
}
