struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        let mut result = vec![];
        let len = nums.len();
        for i in 0..len {
            let mut temp = nums.clone();
            temp.swap(i, len - 1);
            let (last, rest) = temp.split_last().unwrap();
            let mut res = Solution::permute(rest.to_vec());
            for r in &mut res {
                r.push(last.clone());
            }
            result.extend(res)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::permute(vec![1, 2, 3]);
        assert_eq!(result, vec![[]]);
    }
}
