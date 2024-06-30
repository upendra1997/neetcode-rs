struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if target <= 0 || candidates.len() <= 0 {
            return vec![vec![]];
        }
        let first = candidates[0];
        // include first element
        let mut result = Solution::combination_sum(candidates.to_vec(), target - first);
        for r in &mut result {
            r.push(first);
        }
        // don't include first element
        result.extend(Solution::combination_sum(candidates[1..].to_vec(), target));
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
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, vec![[]]);
    }
}
