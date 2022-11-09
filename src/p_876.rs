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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &head;
        while let Some(hr) = cur {
            len += 1;
            cur = &hr.next;
        }

        let mid = len / 2;;
        let mut cur = head;
        let mut idx = 0;
        while let Some(node_box) = cur {
            if idx == mid {
                return Some(node_box);
            }
            cur = node_box.next;
            idx += 1;
        }

        None
    }
}
