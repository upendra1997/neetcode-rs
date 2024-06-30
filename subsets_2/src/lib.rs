struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![nums];
        }
        nums.sort();
        let (mut first, mut rest) = nums.split_first().unwrap();
        // take first
        let mut result1 = Solution::subsets_with_dup(rest.to_vec());
        for i in &mut result1 {
            // // don't insert if value is already present
            // if i.len() > 0 && i[0] == *first {
            //     continue;
            // }
            i.insert(0, first.clone());
        }
        // don't take first
        while rest.len() > 0 && rest[0] == *first {
            let (a, res) = rest.split_first().unwrap();
            rest = res;
        }
        let result2 = Solution::subsets_with_dup(rest.to_vec());
        result1.extend(result2);
        result1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::subsets_with_dup(vec![1, 2, 3]);
        assert_eq!(res, vec![vec![], vec![8]]);
    }
    #[test]
    fn it_works_1() {
        let res = Solution::subsets_with_dup(vec![1, 2, 2, 3]);
        assert_eq!(res, vec![vec![], vec![8]]);
    }
}
