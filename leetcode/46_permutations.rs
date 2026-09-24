// 46. Permutations
// Backtracking with in-place swapping: fix elements position by position,
// recursively generating all sub-permutations, then backtrack by swapping back.
// Time complexity: O(n * n!) where n is the number of elements in nums.
// Space complexity: O(n) auxiliary recursion stack space (excluding output).

struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::backtrack(0, &mut nums, &mut result);
        result
    }

    fn backtrack(first: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            result.push(nums.clone());
            return;
        }
        for i in first..nums.len() {
            nums.swap(first, i);
            Self::backtrack(first + 1, nums, result);
            nums.swap(first, i);
        }
    }
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 3],
        vec![0, 1],
        vec![1],
    ];

    for nums in test_cases {
        println!("Input: nums = {:?}", nums);
        let result = Solution::permute(nums);
        println!("Output: {:?}\n", result);
    }
}
