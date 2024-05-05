use std::iter::repeat;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_right = repeat(0).take(prices.len()).collect::<Vec<i32>>();
        *max_right.last_mut().unwrap() = *prices.last().unwrap();
        for i in (0..prices.len() - 1).rev() {
            max_right[i] = prices[i].max(max_right[i + 1])
        }
        let mut maxi = 0;
        for (i, price) in prices.iter().enumerate() {
            maxi = maxi.max(max_right[i] - price);
        }
        maxi as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::max_profit(prices);
        assert_eq!(result, 5);
    }
}
