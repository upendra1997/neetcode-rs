struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        if target <= 0 || candidates.len() <= 0 {
            return vec![vec![]];
        }
        let first = candidates[0];
        let mut rest = &candidates[1..];
        // include first element
        let mut result = Solution::combination_sum2(rest.to_vec(), target - first);
        for r in &mut result {
            r.insert(0, first);
        }
        // don't include first element
        while rest.len() > 0 && rest[0] == first {
            let (a, res) = rest.split_first().unwrap();
            rest = res;
        }

        result.extend(Solution::combination_sum2(rest.to_vec(), target));
        result
            .into_iter()
            .filter(|vec| vec.iter().sum::<i32>() == target)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = Solution::combination_sum2(candidates, target);
        assert_eq!(result, vec![[]]);
    }

    #[test]
    fn it_works_2() {
        let candidates = vec![2,5,2,1,2];
        let target = 5;
        let result = Solution::combination_sum2(candidates, target);
        assert_eq!(result, vec![[]]);
    }
}
