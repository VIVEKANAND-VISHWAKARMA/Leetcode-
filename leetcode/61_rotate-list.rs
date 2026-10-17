// 61. Rotate List
// First find the list length `n`. With `k = k % n`, if `k == 0` the list is unchanged.
// Otherwise, split the list at index `n - k`, then traverse to the tail of the second
// segment and attach the first segment to it, returning the new head.
// Time complexity: O(n), Space complexity: O(1).

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut len = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        let k = (k as usize) % len;
        if k == 0 {
            return head;
        }

        let steps_to_new_tail = len - k;
        let mut curr = head.as_mut().unwrap();
        for _ in 0..steps_to_new_tail - 1 {
            curr = curr.next.as_mut().unwrap();
        }

        let mut new_head = curr.next.take();

        let mut tail = new_head.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = head;

        new_head
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
    let tests = vec![
        (vec![1, 2, 3, 4, 5], 2),
        (vec![0, 1, 2], 4),
    ];

    for (vals, k) in tests {
        let list = to_list(&vals);
        let rotated = Solution::rotate_right(list, k);
        let result = to_vec(rotated);
        println!("Input: head = {:?}, k = {}", vals, k);
        println!("Output: {:?}", result);
    }
}
