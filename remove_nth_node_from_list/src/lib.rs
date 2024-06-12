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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut second = head.clone();
        let mut prev = &mut head;
        let mut n = n;
        while n > 0 {
            second = second.map(|f| f.next)?;
            n -= 1;
        }
        while second.is_some() {
            prev = &mut prev.as_mut().unwrap().next;
            second = second.and_then(|n| n.next);
        }

        *prev = prev.as_mut().and_then(|f| f.next.take());
        head
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
