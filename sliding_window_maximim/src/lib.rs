use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut window: VecDeque<(i32, usize)> = VecDeque::with_capacity(k as usize);
        let mut maxi = usize::MIN;
        let mut maxv = i32::MIN;
        let mut result = Vec::with_capacity(nums.len());
        for (idx, &v) in nums.iter().enumerate() {
            loop {
                let last = window.back();
                if last.is_none() {
                    break;
                }
                let last = *last.unwrap();
                if last.0 >= v {
                    break;
                }
                window.pop_back();
            }
            window.push_back((v, idx));
            let first = window.front().unwrap();
            if idx as i32 - first.1 as i32 >= k {
                window.pop_front();
            }
            if idx as i32 >= k - 1 {
                result.push(window.front().unwrap().0);
                println!("{:?}", window);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(result, Solution::max_sliding_window(nums, k));
    }
    #[test]
    fn it_works2() {
        let nums = vec![1];
        let k = 1;
        let result = vec![1];
        assert_eq!(result, Solution::max_sliding_window(nums, k));
    }
    #[test]
    fn it_works3() {
        let nums = vec![1, -1];
        let k = 1;
        let result = vec![1, -1];
        assert_eq!(result, Solution::max_sliding_window(nums, k));
    }
    #[test]
    fn it_works4() {
        let nums = vec![1, 1, 1, 1, 1, 4, 5];
        let k = 4;
        let result = vec![1, 1, 4, 5];
        assert_eq!(result, Solution::max_sliding_window(nums, k));
    }
}
