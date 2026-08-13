// Approach: Divide and conquer strategy merging pairs of lists iteratively until one remains.
// Time Complexity: O(N log k), where N is the total number of nodes and k is the number of lists.
// Space Complexity: O(1) auxiliary space, reusing existing nodes.

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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut interval = 1;
        let len = lists.len();
        while interval < len {
            for i in (0..len - interval).step_by(interval * 2) {
                let l1 = lists[i].take();
                let l2 = lists[i + interval].take();
                lists[i] = Self::merge_two_lists(l1, l2);
            }
            interval *= 2;
        }
        lists[0].take()
    }

    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                tail.next = l1;
                tail = tail.next.as_mut().unwrap();
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = next;
            }
        }
        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}

fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn format_list(mut node: Option<&ListNode>) -> String {
    let mut vals = Vec::new();
    while let Some(n) = node {
        vals.push(n.val.to_string());
        node = n.next.as_deref();
    }
    format!("[{}]", vals.join(","))
}

fn format_lists(lists: &[Option<Box<ListNode>>]) -> String {
    let str_vec: Vec<String> = lists.iter().map(|l| format_list(l.as_deref())).collect();
    format!("[{}]", str_vec.join(","))
}

fn main() {
    // Example 1
    let raw_1 = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
    let lists_1: Vec<Option<Box<ListNode>>> = raw_1.iter().map(|v| build_list(v)).collect();
    println!("Input 1:  {}", format_lists(&lists_1));
    let merged_1 = Solution::merge_k_lists(lists_1);
    println!("Output 1: {}\n", format_list(merged_1.as_deref()));

    // Example 2
    let lists_2: Vec<Option<Box<ListNode>>> = vec![];
    println!("Input 2:  {}", format_lists(&lists_2));
    let merged_2 = Solution::merge_k_lists(lists_2);
    println!("Output 2: {}\n", format_list(merged_2.as_deref()));

    // Example 3
    let raw_3 = vec![vec![]];
    let lists_3: Vec<Option<Box<ListNode>>> = raw_3.iter().map(|v| build_list(v)).collect();
    println!("Input 3:  {}", format_lists(&lists_3));
    let merged_3 = Solution::merge_k_lists(lists_3);
    println!("Output 3: {}", format_list(merged_3.as_deref()));
}
