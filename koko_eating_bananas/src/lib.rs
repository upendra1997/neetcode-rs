use std::ops::Div;

struct Solution;

impl Solution {
    fn eating_speed(piles: &[i32], k: usize, h: i32) -> bool {
        let result = piles
            .iter()
            .map(|val| 1.0 * (*val) as f64)
            .map(|v| v.div(k as f64).ceil())
            .sum::<f64>();
        // println!("result {}: {}", h, result);
        result <= h as f64
    }
    pub fn min_eating_speed(piles: Vec<i32>, fin: i32) -> i32 {
        let mut l = 1;
        let mut h = *piles.iter().max().unwrap();
        let mut m = 0;
        let mut last_result = true;
        while l < h {
            m = l + ((h - l) / 2);
            last_result = Solution::eating_speed(&piles, m as usize, fin);
            // println!("{} - {} = {}: {}", l, h, m, last_result);
            if last_result {
                h = m;
            } else {
                l = m + 1;
            }
        }
        if last_result == false {
            return m + 1;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::min_eating_speed(vec![3, 6, 7, 11], 8);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works1() {
        let result = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(result, 30);
    }
    #[test]
    fn it_works2() {
        let result = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6);
        assert_eq!(result, 23);
    }

    #[test]
    fn it_works3() {
        let result = Solution::min_eating_speed(vec![312884470], 312884469);
        assert_eq!(result, 2);
    }
}
