use std::ops::Div;

struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut result = position
            .into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| (p, (((target - p) as f64) / s as f64)))
            .collect::<Vec<_>>();
        result.sort_by(|x, y| {
            let eq = x.0.cmp(&y.0);
            match eq {
                std::cmp::Ordering::Equal => x.1.partial_cmp(&y.1).unwrap(),
                _ => eq,
            }
        });
        println!("{:?}", result);
        let mut res = 0;
        let mut maxi = f64::MIN;
        for i in (0..result.len()).rev() {
            if result[i].1 > maxi {
                maxi = result[i].1;
                res += 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let solution = Solution::car_fleet(10, vec![6, 8], vec![3, 2]);
        assert_eq!(solution, 2);
    }

    #[test]
    fn it_works_2() {
        let solution = Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
        assert_eq!(solution, 3);
    }

    #[test]
    fn it_works_3() {
        let solution = Solution::car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]);
        assert_eq!(solution, 1);
    }
}
