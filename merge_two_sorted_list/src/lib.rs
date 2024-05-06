// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut pointer = &mut head;
        while list1.is_some() && list2.is_some() {
            let smaller_value = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                &mut list1
            } else {
                &mut list2
            };
            pointer.next = smaller_value.take();
            pointer = pointer.next.as_mut().unwrap();
            *smaller_value = pointer.next.take();
        }
        pointer.next = if list1.is_some() { list1 } else { list2 };
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut elem1 = ListNode::new(1);
        let mut elem2 = ListNode::new(2);
        let mut elem3 = ListNode::new(4);
        elem2.next = Some(Box::new(elem3));
        elem1.next = Some(Box::new(elem2));
        let list1 = Box::new(elem1);
        let mut elem1 = ListNode::new(1);
        let mut elem2 = ListNode::new(3);
        let mut elem3 = ListNode::new(4);
        elem2.next = Some(Box::new(elem3));
        elem1.next = Some(Box::new(elem2));
        let list2 = Box::new(elem1);
        let result = Solution::merge_two_lists(Some(list1), Some(list2));
        println!("{:?}", result);
        assert_eq!(3, 4);
    }
}
