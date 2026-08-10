// Approach: Recursively compare the heads of both lists, attach the smaller
// node to the merged result of the remaining lists, and return the combined head.
// Time Complexity: O(n + m) where n and m are the lengths of list1 and list2.
// Space Complexity: O(n + m) for the recursion call stack.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, list) | (list, None) => list,
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    Some(node2)
                }
            }
        }
    }
}

fn create_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in nums.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vals = Vec::new();
    while let Some(node) = head {
        vals.push(node.val);
        head = node.next;
    }
    vals
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
    ];

    for (arr1, arr2) in test_cases {
        let list1 = create_list(&arr1);
        let list2 = create_list(&arr2);
        println!("Input: list1 = {:?}, list2 = {:?}", arr1, arr2);
        let merged = Solution::merge_two_lists(list1, list2);
        println!("Output: {:?}", list_to_vec(merged));
        println!();
    }
}
