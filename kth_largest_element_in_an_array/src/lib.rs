use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
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
        vec.pop().unwrap().0
    }
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
