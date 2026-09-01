// 30. Substring with Concatenation of All Words
// Sliding window with word_len different starting offsets (0..word_len).
// For each offset, maintain a word count map over a window of length num_words * word_len,
// expanding the right boundary and shrinking the left boundary whenever counts exceed targets.
// Time Complexity: O(s.len() * word_len), Space Complexity: O(words.len() * word_len).

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return Vec::new();
        }

        let word_len = words[0].len();
        let num_words = words.len();
        let total_len = word_len * num_words;
        let s_len = s.len();

        if s_len < total_len {
            return Vec::new();
        }

        let mut word_counts: HashMap<&str, usize> = HashMap::new();
        for word in &words {
            *word_counts.entry(word.as_str()).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        let s_str = s.as_str();

        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut current_counts: HashMap<&str, usize> = HashMap::new();
            let mut matched_words = 0;

            while right + word_len <= s_len {
                let sub = &s_str[right..right + word_len];
                right += word_len;

                if let Some(&target_count) = word_counts.get(sub) {
                    let count = current_counts.entry(sub).or_insert(0);
                    *count += 1;
                    matched_words += 1;

                    while *current_counts.get(sub).unwrap() > target_count {
                        let left_sub = &s_str[left..left + word_len];
                        *current_counts.get_mut(left_sub).unwrap() -= 1;
                        matched_words -= 1;
                        left += word_len;
                    }

                    if matched_words == num_words {
                        result.push(left as i32);
                    }
                } else {
                    current_counts.clear();
                    matched_words = 0;
                    left = right;
                }
            }
        }

        result
    }
}

fn main() {
    let test_cases = vec![
        (
            "barfoothefoobarman",
            vec!["foo", "bar"],
        ),
        (
            "wordgoodgoodgoodbestword",
            vec!["word", "good", "best", "word"],
        ),
        (
            "barfoofoobarthefoobarman",
            vec!["bar", "foo", "the"],
        ),
    ];

    for (s, words) in test_cases {
        let words_vec: Vec<String> = words.into_iter().map(|w| w.to_string()).collect();
        let ans = Solution::find_substring(s.to_string(), words_vec.clone());
        println!("s: \"{}\"", s);
        println!("words: {:?}", words_vec);
        println!("Output: {:?}\n", ans);
    }
}
