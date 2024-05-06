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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut next = head;
        while next != None {
            let mut next_ = next.unwrap();
            let next_next = next_.next;
            (*next_).next = prev;
            prev = Some(next_);
            next = next_next;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut elem1 = ListNode::new(1);
        let mut elem2 = ListNode::new(2);
        let mut elem3 = ListNode::new(3);
        let mut elem4 = ListNode::new(4);
        let mut elem5 = ListNode::new(5);
        elem4.next = Some(Box::new(elem5));
        elem3.next = Some(Box::new(elem4));
        elem2.next = Some(Box::new(elem3));
        elem1.next = Some(Box::new(elem2));
        println!("{:?}", elem1);
        let mut result = Solution::reverse_list(Some(Box::new(elem1)));
        println!("{:?}", result);
        assert_eq!(3, 4);
    }
}
