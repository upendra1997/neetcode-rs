use std::usize;

struct Solution;

#[derive(Clone)]
struct Item {
    left_mark: usize,
    height: usize,
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut maximum = 0;
        let n = heights.len();
        for (cur_index, height) in heights.into_iter().enumerate() {
            let mut start = cur_index;
            while stack.len() > 0
                && stack.last().map(|x: &Item| x.clone()).unwrap().height > height as usize
            {
                let item = stack.pop().unwrap();
                maximum = maximum.max((cur_index - item.left_mark) * item.height);
                start = item.left_mark;
            }
            stack.push(Item {
                left_mark: start,
                height: height as usize,
            });
        }
        for i in stack {
            maximum = maximum.max(i.height * (n - i.left_mark))
        }
        maximum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(result, 10);
    }

    #[test]
    fn it_works1() {
        let result = Solution::largest_rectangle_area(vec![2, 4]);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works2() {
        let result = Solution::largest_rectangle_area(vec![4, 2]);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works3() {
        let result = Solution::largest_rectangle_area(vec![1, 1]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works4() {
        let result = Solution::largest_rectangle_area(vec![2, 1, 2]);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_works5() {
        let result = Solution::largest_rectangle_area(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(result, 6);
    }
}
