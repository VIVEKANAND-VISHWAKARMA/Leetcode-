// 47. Permutations II
// Sort the input array so identical elements are adjacent to each other.
// Use backtracking with a visited array, pruning duplicates by skipping nums[i]
// if nums[i] == nums[i - 1] and nums[i - 1] was not used in the current path.
// Time complexity: O(N * N!), Space complexity: O(N) auxiliary space.

struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        Self::backtrack(&nums, &mut used, &mut current, &mut result);
        result
    }

    fn backtrack(
        nums: &[i32],
        used: &mut [bool],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            current.push(nums[i]);
            Self::backtrack(nums, used, current, result);
            current.pop();
            used[i] = false;
        }
    }
}

fn main() {
    let test_cases = vec![vec![1, 1, 2], vec![1, 2, 3]];

    for nums in test_cases {
        println!("Input: nums = {:?}", nums);
        let output = Solution::permute_unique(nums);
        println!("Output: {:?}", output);
    }
}
