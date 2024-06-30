use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn cross_product<T: Clone + Ord>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
        if a.len() == 0 {
            return vec![];
        }
        if a.len() == 1 {
            return a[0].clone().into_iter().map(|x| vec![x]).collect();
        }
        let (mut first, mut rest) = a.split_first().unwrap();
        let mut result = Solution::cross_product(rest.to_vec());
        let mut res = vec![];
        for e in first {
            for ele in &mut result {
                let mut r = ele.clone();
                r.insert(0, e.clone());
                res.push(r);
            }
        }
        res
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map: HashMap<u32, Vec<char>> = HashMap::new();
        map.insert(2, vec!['a', 'b', 'c']);
        map.insert(3, vec!['d', 'e', 'f']);
        map.insert(4, vec!['g', 'h', 'i']);
        map.insert(5, vec!['j', 'k', 'l']);
        map.insert(6, vec!['m', 'n', 'o']);
        map.insert(7, vec!['p', 'q', 'r', 's']);
        map.insert(8, vec!['t', 'u', 'v']);
        map.insert(9, vec!['w', 'x', 'y', 'z']);
        let array = digits
            .chars()
            .map(|c| map.get(&c.to_digit(10).unwrap()).unwrap().clone())
            .collect::<Vec<_>>();
        Solution::cross_product(array)
            .into_iter()
            .map(|arr| {
                let mut r = String::new();
                for x in arr {
                    r.push(x);
                }
                r
            })
            .collect()
        // println!("{:?}", result);
        // vec![]
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
        let result = Solution::letter_combinations("23".to_string());
        assert_eq!(result, vec!["".to_string()]);
    }
}
