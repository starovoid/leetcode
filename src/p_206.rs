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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut data = Vec::new();
        let mut cur = head.as_ref();
        while cur.is_some() {
            let v = (**cur.as_ref().unwrap()).val;
            data.push(v);
            cur = (**cur.as_ref().unwrap()).next.as_ref();
        }
        
        if data.len() == 0 {
            return None;
        }
        
        let mut new_head = ListNode::new(data[0]);
        for x in data.into_iter().skip(1) {
            new_head = ListNode {
                val: x,
                next: Some(Box::new(new_head)),
            };
        }
        
        Some(Box::new(new_head))
    }
}
