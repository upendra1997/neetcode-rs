use std::{cmp::Reverse, collections::BinaryHeap, ops::Sub};

#[derive(Debug)]
struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if *self.small.peek().unwrap_or(&i32::MAX) >= num {
            self.small.push(num);
        } else {
            self.large.push(Reverse(num));
        }
        while self.small.len().abs_diff(self.large.len()) > 1 {
            match self.small.len().cmp(&self.large.len()) {
                std::cmp::Ordering::Less => self.small.push(self.large.pop().unwrap().0),
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Greater => self.large.push(Reverse(self.small.pop().unwrap())),
            };
        }
    }

    fn find_median(&self) -> f64 {
        println!("{:?}", self);
        match self.small.len().cmp(&self.large.len()) {
            std::cmp::Ordering::Less => self.large.peek().unwrap().0 as f64,
            std::cmp::Ordering::Equal => {
                (self.small.peek().unwrap() + self.large.peek().unwrap().0) as f64 / 2.0
            }
            std::cmp::Ordering::Greater => *self.small.peek().unwrap() as f64,
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        println!("{}", median_finder.find_median());
        median_finder.add_num(2); // arr = [1, 2]
        println!("{}", median_finder.find_median());
        median_finder.add_num(3); // arr[1, 2, 3]
        println!("{}", median_finder.find_median());
        assert_eq!(5, 4);
    }
}
