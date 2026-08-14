// Approach: Recursively detach the first two nodes, recursively swap the remaining
// list, and re-link the second node to point to the first node, whose next points
// to the swapped remainder.
// Time Complexity: O(n) where n is the number of nodes.
// Space Complexity: O(n) stack space due to recursion (n <= 100).

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut first) => match first.next.take() {
                Some(mut second) => {
                    first.next = Self::swap_pairs(second.next.take());
                    second.next = Some(first);
                    Some(second)
                }
                None => Some(first),
            },
            None => None,
        }
    }
}

fn to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let test_cases: Vec<Vec<i32>> = vec![
        vec![1, 2, 3, 4],
        vec![],
        vec![1],
        vec![1, 2, 3],
    ];

    for input in test_cases {
        let list = to_list(&input);
        let swapped = Solution::swap_pairs(list);
        let output = to_vec(swapped);
        println!("Input: {:?} => Output: {:?}", input, output);
    }
}
