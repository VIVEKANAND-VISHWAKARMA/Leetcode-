// Approach: Binary search on the smaller array to partition both arrays such that
// the left partition contains half the elements and all left elements <= right elements.
// Time Complexity: O(log(min(m, n)))
// Space Complexity: O(1)

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let m = nums1.len();
        let n = nums2.len();
        let half_len = (m + n + 1) / 2;

        let mut low = 0;
        let mut high = m;

        while low <= high {
            let i = (low + high) / 2;
            let j = half_len - i;

            let max_left1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let min_right1 = if i == m { i32::MAX } else { nums1[i] };

            let max_left2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let min_right2 = if j == n { i32::MAX } else { nums2[j] };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                if (m + n) % 2 == 1 {
                    return max_left1.max(max_left2) as f64;
                } else {
                    let left_max = max_left1.max(max_left2) as f64;
                    let right_min = min_right1.min(min_right2) as f64;
                    return (left_max + right_min) / 2.0;
                }
            } else if max_left1 > min_right2 {
                high = i - 1;
            } else {
                low = i + 1;
            }
        }

        0.0
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    println!("nums1 = {:?}, nums2 = {:?}", nums1, nums2);
    let median1 = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Output: {:.5}", median1);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    println!("nums1 = {:?}, nums2 = {:?}", nums1, nums2);
    let median2 = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Output: {:.5}", median2);
}
