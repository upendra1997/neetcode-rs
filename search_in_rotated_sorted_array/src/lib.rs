struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut h = nums.len();
        let mut m = 0;
        while l < h {
            m = l + (h - l) / 2;
            println!("{}-{}: {}", l, h, m);
            if nums[m] == target {
                break;
            }
            if nums[m] >= nums[0] {
                if target >= nums[0] && target <= nums[m] {
                    h = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if target >= nums[m] && target <= nums[nums.len() - 1] {
                    l = m + 1;
                } else {
                    h = m - 1;
                }
            }
        }
        m = l + (h - l) / 2;
        println!("{}-{}: {}", l, h, m);
        if m < nums.len() && nums[m] == target {
            m as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = 4;
        assert_eq!(Solution::search(nums, target), result);
    }
    #[test]
    fn it_works2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let result = -1;
        assert_eq!(Solution::search(nums, target), result);
    }
    #[test]
    fn it_works3() {
        let nums = vec![1];
        let target = 0;
        let result = -1;
        assert_eq!(Solution::search(nums, target), result);
    }

    #[test]
    fn it_works4() {
        let nums = vec![3, 1];
        let target = 3;
        let result = 0;
        assert_eq!(Solution::search(nums, target), result);
    }
}
