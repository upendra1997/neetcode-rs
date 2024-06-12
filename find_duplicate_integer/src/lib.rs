use std::usize;

struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        while fast != slow {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }
        let mut new_slow = slow;
        let mut slow = 0;
        while new_slow != slow {
            slow = nums[slow] as usize;
            new_slow = nums[new_slow] as usize;
        }
        return slow as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }
    #[test]
    fn it_works_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }
    #[test]
    fn it_works_2() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }
}
