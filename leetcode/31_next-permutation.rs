// 31. Next Permutation
// Approach: Traverse from right to find the first index i where nums[i] < nums[i + 1].
// If found, find the first element nums[j] from the right greater than nums[i], swap them,
// and reverse the suffix starting at index i + 1. If no such i exists, reverse the entire array.
// Time complexity: O(n), Space complexity: O(1).

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let mut i = (n - 2) as isize;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = n - 1;
            while nums[j] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j);
        }

        nums[((i + 1) as usize)..].reverse();
    }
}

fn main() {
    let mut test1 = vec![1, 2, 3];
    print!("Input: nums = {:?} -> ", test1);
    Solution::next_permutation(&mut test1);
    println!("Output: {:?}", test1);

    let mut test2 = vec![3, 2, 1];
    print!("Input: nums = {:?} -> ", test2);
    Solution::next_permutation(&mut test2);
    println!("Output: {:?}", test2);

    let mut test3 = vec![1, 1, 5];
    print!("Input: nums = {:?} -> ", test3);
    Solution::next_permutation(&mut test3);
    println!("Output: {:?}", test3);
}
