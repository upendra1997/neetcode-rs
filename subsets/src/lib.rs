use std::ops::BitAnd;

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let final_size: usize = (2 as usize).checked_pow(nums.len() as u32).unwrap();
        let mut res = Vec::with_capacity(final_size);
        for i in 0..final_size {
            let mut sub_result = Vec::with_capacity(nums.len());
            for j in 0..nums.len() {
                if i.bitand(1 << j) != 0 {
                    sub_result.push(nums[j]);
                }
            }
            res.push(sub_result)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::subsets(vec![8, 10]);
        assert_eq!(res, vec![vec![], vec![8]]);
    }
}
