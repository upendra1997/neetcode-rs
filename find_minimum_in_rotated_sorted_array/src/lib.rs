use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let part = nums.binary_search_by(|&x| {
            if x < nums[0] {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        nums[part.unwrap_err().rem_euclid(nums.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let nums = vec![3, 4, 5, 1, 2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let result = Solution::find_min(nums);
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works3() {
        let nums = vec![11, 13, 15, 17];
        let result = Solution::find_min(nums);
        assert_eq!(result, 11);
    }
}
