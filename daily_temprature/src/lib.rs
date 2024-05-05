struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut indexes = Vec::with_capacity(temperatures.len());
        let mut result = Vec::new();
        result.resize_with(temperatures.len(), Default::default);
        for (idx, temp) in temperatures.into_iter().enumerate() {
            while let Some((old_idx, old_temp)) = indexes.last() {
                if *old_temp < temp {
                    result[*old_idx as usize] = (idx - *old_idx) as i32;
                    indexes.pop().unwrap();
                } else {
                    break;
                }
            }
            indexes.push((idx, temp));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let temp = vec![73, 74, 75, 71, 69, 72, 76, 73];

        assert_eq!(Solution::daily_temperatures(temp), vec![]);
    }
}
