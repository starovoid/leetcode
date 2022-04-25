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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut cur = head;
        let mut items = Vec::new();
        
        while let Some(node) = cur {
            items.push(node.val);
            cur = node.next;
        }
        
        for i in 0..items.len()/2 {
            if items[i] != items[items.len() - i - 1] {
                return false;
            }
        }
        
        true
    }
}
