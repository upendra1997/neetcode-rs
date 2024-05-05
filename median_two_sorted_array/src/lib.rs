use std::{mem, ops::Rem};

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1.as_slice();
        let mut nums2 = nums2.as_slice();
        if nums2.len() < nums1.len() {
            mem::swap(&mut nums2, &mut nums1);
        }
        let mut l = 0;
        let mut h = nums1.len();
        let mut m = l + ((h - l) / 2);
        let N = nums1.len() + nums2.len();
        let half = N / 2;
        let mut other = half - m;
        let mut nums1_left = f64::MIN;
        let mut nums2_left = if other == 0 {
            f64::MIN
        } else {
            nums2[other - 1] as f64
        };
        let mut nums1_right = if m >= nums1.len() {
            f64::MAX
        } else {
            nums1[m] as f64
        };
        let mut nums2_right = if other >= nums2.len() {
            f64::MAX
        } else {
            nums2[other] as f64
        };
        loop {
            m = l + ((h - l) / 2);
            other = half - m;
            nums1_left = if m == 0 {
                f64::MIN
            } else {
                nums1[m - 1] as f64
            };
            nums1_right = if m >= nums1.len() {
                f64::MAX
            } else {
                nums1[m] as f64
            };
            nums2_left = if other == 0 {
                f64::MIN
            } else {
                nums2[other - 1] as f64
            };
            nums2_right = if other >= nums2.len() {
                f64::MAX
            } else {
                nums2[other] as f64
            };
            println!(
                "nums1: {}|{} nums2: {}|{}",
                nums1_left, nums1_right, nums2_left, nums2_right
            );
            if nums1_left <= nums2_right && nums2_left <= nums1_right {
                break;
            } else if nums1_left > nums2_right {
                h = m - 1;
            } else {
                l = m + 1;
            }
        }
        if N.rem(2) == 0 {
            (nums1_left.max(nums2_left) + nums2_right.min(nums1_right)) / 2.0
        } else {
            nums1_right.min(nums2_right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums1 = vec![];
        let nums2 = vec![];
        let res = 0.0;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let res = 2.0;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn it_works2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let res = 2.5;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn it_works3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let res = 1.0;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn it_works4() {
        let nums1 = vec![1];
        let nums2 = vec![1];
        let res = 1.0;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn it_works5() {
        let nums1 = vec![1];
        let nums2 = vec![2, 3, 4];
        let res = 2.5;
        assert_eq!(res, Solution::find_median_sorted_arrays(nums1, nums2));
    }
}
