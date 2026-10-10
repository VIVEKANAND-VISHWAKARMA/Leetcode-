// 57. Insert Interval
// Approach: Iterate through intervals in three phases: add intervals that end before
// newInterval starts, merge all overlapping intervals into newInterval by expanding
// its bounds, and finally append all remaining intervals that start after newInterval ends.
// Time Complexity: O(n) single pass through intervals.
// Space Complexity: O(n) for the output list (O(1) auxiliary space).

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut i = 0;
        let n = intervals.len();
        let mut new_interval = new_interval;

        // 1. Add all intervals coming before new_interval
        while i < n && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }

        // 2. Merge all overlapping intervals
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);

        // 3. Add all intervals coming after new_interval
        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

fn main() {
    // Example 1
    let intervals1 = vec![vec![1, 3], vec![6, 9]];
    let new_interval1 = vec![2, 5];
    println!("Input: intervals = {:?}, newInterval = {:?}", intervals1, new_interval1);
    let result1 = Solution::insert(intervals1, new_interval1);
    println!("Output: {:?}\n", result1);

    // Example 2
    let intervals2 = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval2 = vec![4, 8];
    println!("Input: intervals = {:?}, newInterval = {:?}", intervals2, new_interval2);
    let result2 = Solution::insert(intervals2, new_interval2);
    println!("Output: {:?}", result2);
}
