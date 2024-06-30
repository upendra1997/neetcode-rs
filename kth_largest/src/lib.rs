use std::{cmp::Reverse, collections::BinaryHeap, usize};

struct KthLargest {
    vec: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut vec = BinaryHeap::with_capacity(k as usize);
        for i in nums {
            if vec.len() < vec.capacity() {
                vec.push(Reverse(i));
            } else {
                if i >= vec.peek().unwrap().0 {
                    vec.pop();
                    vec.push(Reverse(i));
                }
            }
        }
        KthLargest { vec }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.vec.len() < self.vec.capacity() {
            self.vec.push(Reverse(val));
        } else {
            if val >= self.vec.peek().unwrap().0 {
                self.vec.pop();
                self.vec.push(Reverse(val));
            }
        }
        self.vec.peek().unwrap().0.clone()
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let obj = KthLargest::new(4, vec![4, 5, 8, 2]);
        let ret_1: i32 = obj.add(1);
        assert_eq!(ret_1, 4);
    }
}
