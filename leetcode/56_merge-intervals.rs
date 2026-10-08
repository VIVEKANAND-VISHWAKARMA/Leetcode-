// 56. Merge Intervals
// Approach: Sort the intervals by their start times. Iterate through them, merging each
// interval with the last one in the result list if they overlap (current start <= previous end),
// updating the end to the maximum of both. Otherwise, append the interval as a new entry.
// Time Complexity: O(n log n) due to sorting, where n is the number of intervals.
// Space Complexity: O(n) to store the merged intervals (and O(log n) sorting space).

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }

        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if interval[0] <= last[1] {
                    last[1] = max(last[1], interval[1]);
                } else {
                    merged.push(interval);
                }
            } else {
                merged.push(interval);
            }
        }

        merged
    }
}

fn main() {
    let test_cases = vec![
        vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
        vec![vec![1, 4], vec![4, 5]],
        vec![vec![4, 7], vec![1, 4]],
    ];

    for intervals in test_cases {
        println!("Input:  {:?}", intervals);
        let output = Solution::merge(intervals);
        println!("Output: {:?}", output);
        println!();
    }
}
