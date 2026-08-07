// Approach: Recursive traversal to count nodes from the end of the list.
// Upon unwinding, when count reaches n + 1, the current node directly precedes
// the node to remove, allowing us to update its `next` pointer to skip the target node.
// Time Complexity: O(N) single pass, where N is the length of the list.
// Space Complexity: O(N) call stack space.

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        Self::helper(&mut dummy, n);
        dummy.unwrap().next
    }

    fn helper(node: &mut Option<Box<ListNode>>, n: i32) -> i32 {
        if let Some(curr) = node {
            let count = Self::helper(&mut curr.next, n) + 1;
            if count == n + 1 {
                let next = curr.next.as_mut().unwrap().next.take();
                curr.next = next;
            }
            count
        } else {
            0
        }
    }
}

fn create_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 2),
        (vec![1], 1),
        (vec![1, 2], 1),
    ];

    for (arr, n) in test_cases {
        let head = create_list(&arr);
        println!("Input: head = {:?}, n = {}", arr, n);
        let result = Solution::remove_nth_from_end(head, n);
        println!("Output: {:?}", list_to_vec(result));
    }
}
