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

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut pointer = &mut head;
        let mut b = true;
        while list1.is_some() && list2.is_some() {
            let smaller_value = if b { &mut list1 } else { &mut list2 };
            b = !b;
            pointer.next = smaller_value.take();
            pointer = pointer.next.as_mut().unwrap();
            *smaller_value = pointer.next.take();
        }
        pointer.next = if list1.is_some() { list1 } else { list2 };
        head.next
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut dummy_first = Box::new(ListNode::new(0));
        let mut count = 0;
        {
            let mut fast = head.as_mut().unwrap();
            while fast.next.is_some() {
                count += 1;
                fast = fast.next.as_mut().unwrap();
            }
        }
        let mut reversed_list = None;
        {
            count /= 2;
            let mut fast = head.as_mut().unwrap();
            while count > 0 {
                fast = fast.next.as_mut().unwrap();
                count -= 1;
            }
            reversed_list = Solution::reverse_list(fast.next.take());
        }
        let mut new_head = Box::new(ListNode::new(0));
        new_head.next = head.take();
        *head = Solution::merge_two_lists(new_head.next, reversed_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2;
        assert_eq!(result, 4);
    }
}
