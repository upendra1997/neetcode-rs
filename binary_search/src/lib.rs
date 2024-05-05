use std::usize;

struct Solution;
impl Solution {
    pub fn binary_search(nums: &[i32], pos: usize, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mid = nums.len() / 2;
        let left = &nums[..mid];
        let right = &nums[mid + 1..];
        if nums[mid] == target {
            return (mid + pos) as i32;
        } else if nums[mid] < target {
            return Solution::binary_search(right, pos + mid + 1, target);
        } else {
            return Solution::binary_search(left, pos, target);
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::binary_search(&nums, 0, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
        assert_eq!(result, -1);
    }
    #[test]
    fn it_works1() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(result, 4);
    }
}
