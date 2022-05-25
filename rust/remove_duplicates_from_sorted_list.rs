// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// 83. Remove Duplicates from Sorted List

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut h = head;
        let mut p1 = h.as_mut().unwrap();
        while let Some(p2) = p1.next.as_mut() {
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }
        h
    }
}

impl SolutionRecursive {
    fn remove(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match node {
            Some(node) => {
                if let Some(next_node) = &node.next {
                    if node.val == next_node.val {
                        Solution::remove(node.next)
                    } else {
                        Some(Box::new(ListNode{ val: node.val, next: Solution::remove(node.next) }))
                    }
                } else {
                    Some(Box::new(ListNode::new(node.val)))
                }
            }
            None => None
        }
    }
    
    
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::remove(head)
    }
}
