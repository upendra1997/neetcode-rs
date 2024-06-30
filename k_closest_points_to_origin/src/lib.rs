use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut vec = BinaryHeap::with_capacity(k as usize);
        for i in points {
            let distance_from_origin = i[0] * i[0] + i[1] * i[1];
            if vec.len() < vec.capacity() {
                vec.push((distance_from_origin, i));
            } else {
                if distance_from_origin < vec.peek().unwrap().0 {
                    vec.pop();
                    vec.push((distance_from_origin, i));
                }
            }
        }
        let mut result = Vec::with_capacity(k as usize);
        while vec.len() > 0 {
            result.push(vec.pop().unwrap().1);
        }
        result
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
