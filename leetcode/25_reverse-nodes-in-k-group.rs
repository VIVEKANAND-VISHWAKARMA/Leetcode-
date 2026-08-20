// Approach: Count k nodes to ensure a full group exists. Reverse the first k nodes iteratively,
// recursively call reverse_k_group on the remainder, and attach the result to the group's tail.
// Time Complexity: O(n) where n is the number of nodes.
// Space Complexity: O(n/k) due to the recursion call stack.

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut cur = &head;
        while count < k && cur.is_some() {
            cur = &cur.as_ref().unwrap().next;
            count += 1;
        }
        if count < k {
            return head;
        }

        let mut cur = head;
        let mut prev = None;
        for _ in 0..k {
            let next_node = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = prev;
            prev = cur;
            cur = next_node;
        }

        let rest = Self::reverse_k_group(cur, k);
        let mut tail = &mut prev;
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = rest;
                break;
            }
            tail = &mut node.next;
        }
        prev
    }
}

fn vec_to_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 2),
        (vec![1, 2, 3, 4, 5], 3),
    ];

    for (vals, k) in test_cases {
        print!("Input: head = {:?}, k = {} -> ", vals, k);
        let head = vec_to_list(&vals);
        let result = Solution::reverse_k_group(head, k);
        let result_vec = list_to_vec(result);
        println!("Output: {:?}", result_vec);
    }
}
