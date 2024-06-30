use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in stones {
            heap.push(i);
        }
        while heap.len() >= 2 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if y != x {
                heap.push(y - x);
            }
        }
        heap.pop().unwrap_or(0)
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
