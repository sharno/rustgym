// https://leetcode.com/problems/reverse-linked-list/description/

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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    while let Some(mut node) = current {
        let temp = node.next.take();
        node.next = prev;
        prev = Some(node.clone());
        current = temp;
    }
    prev
}
