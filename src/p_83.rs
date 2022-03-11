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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        
        let mut new_head = ListNode::new(-101);
        let mut tail = &mut new_head;
        
        let mut cur = &mut head;
        let mut temp: Option<Box<ListNode>>;
        
        while cur.is_some() {
            if cur.as_ref().unwrap().val != tail.val {
                tail.next = cur.take();
                tail = tail.next.as_mut().unwrap();
                cur = &mut tail.next;
            } else {
                temp = cur.take().unwrap().next;
                cur = &mut temp;
            }
        }
        
        new_head.next
    }
}
