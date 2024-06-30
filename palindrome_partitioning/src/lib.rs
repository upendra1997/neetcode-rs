use std::collections::HashSet;

struct Solution;

impl Solution {
    fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let arr = s.chars().into_iter().collect::<Vec<char>>();
        let mut start = 0;
        let mut end = s.len() - 1;
        while start < end {
            if arr[start] != arr[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        return true;
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        if s.len() == 0 {
            return vec![vec![]];
        }
        if s.len() == 1 {
            return vec![vec![s]];
        }
        let mut result = vec![];
        for i in 1..=s.len() {
            if Solution::is_palindrome(s[0..i].to_string()) {
                let palindrome = s[0..i].to_string();
                let rest = s[i..].to_string();
                let mut rests = Solution::partition(rest.clone());
                // println!("string: {} palindrome: {} rest: {} result: {:?}", s, palindrome, rest, rests);
                for i in &mut rests {
                    i.insert(0, palindrome.clone())
                }
                result.extend(rests);
            }
        }
        // println!("{} {:?}", s, result);
        return result;
    }
}

// impl Solution {
//     fn isPalindrome(s: String) -> bool {
//         if s.len() <= 1 {
//             return true;
//         }
//         let arr = s.chars().into_iter().collect::<Vec<char>>();
//         let mut start = 0;
//         let mut end = s.len() - 1;
//         while start < end {
//             if arr[start] != arr[end] {
//                 return false;
//             }
//             start += 1;
//             end -= 1;
//         }
//         return true;
//     }

//     fn join_interval(
//         mut interval1: (usize, usize),
//         mut interval2: (usize, usize),
//     ) -> Option<(usize, usize)> {
//         if interval1.0 > interval2.0 {
//             std::mem::swap(&mut interval1, &mut interval2);
//         }
//         if interval1.1 <= interval2.0 {
//             return None;
//         }
//         return Some((interval1.0.min(interval2.0), interval1.1.max(interval2.1)));
//     }

//     fn join(intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
//         let result = intervals.into_iter().fold(vec![], |mut accum, ele| {
//             if accum.len() == 0 {
//                 accum.push(ele);
//                 return accum;
//             }
//             match Solution::join_interval(accum.last().unwrap().clone(), ele) {
//                 Some(result) => {
//                     *accum.last_mut().unwrap() = result;
//                 }
//                 None => {
//                     accum.push(ele);
//                 }
//             }
//             accum
//         });
//         result
//     }

//     fn partition_at_index(idx: usize, s: String) -> HashSet<Vec<(usize, usize)>> {
//         let arr = s.chars().into_iter().collect::<Vec<char>>();
//         let mut low = idx as isize;
//         let mut high = idx + 1;
//         let mut result = vec![];
//         while low >= 0 && high <= s.len() {
//             if arr[low as usize] == arr[high - 1] {
//                 match result.binary_search(&(low as usize, high)) {
//                     Ok(n) | Err(n) => {
//                         result.insert(n, (low as usize, high));
//                     }
//                 }
//                 low -= 1;
//                 high += 1;
//             } else {
//                 break;
//             }
//         }
//         let mut low = (idx as isize - 1) as isize;
//         let mut high = idx + 1;
//         while low >= 0 && high <= s.len() {
//             if arr[low as usize] == arr[high - 1] {
//                 match result.binary_search(&(low as usize, high)) {
//                     Ok(n) | Err(n) => {
//                         result.insert(n, (low as usize, high));
//                     }
//                 }
//                 low -= 1;
//                 high += 1;
//             } else {
//                 break;
//             }
//         }
//         let mut some = vec![];
//         for x in result {
//             let mut rest = Solution::partition_at_index(x.1, s.clone());
//             // rest.insert(0, vec![x]);
//             let mut t = HashSet::new();
//             for mut r in rest.into_iter() {
//                 r.insert(0, x);
//                 t.insert(r);
//             }
//             rest = t;
//             if rest.len() == 0 {
//                 let mut t = HashSet::new();
//                 t.insert(vec![x]);
//                 rest = t;
//             }
//             // println!("{:?}: {:?}", &s[x.0..x.1], rest);
//             some.extend(rest);
//         }
//         some.into_iter().map(|x| Solution::join(x)).collect()
//     }

//     pub fn partition(s: String) -> Vec<Vec<String>> {
//         if s.len() == 0 {
//             return vec![vec![]];
//         }
//         let result = Solution::partition_at_index(0, s.clone());
//         result
//             .into_iter()
//             .map(|x| {
//                 x.into_iter()
//                     .map(|c| s[c.0..c.1].to_string())
//                     .filter(|s| Solution::isPalindrome(s.to_string()))
//                     .collect()
//             })
//             .filter(|x: &Vec<String>| {
//                 let mut chars = "".to_string();
//                 for i in x {
//                     for c in i.chars() {
//                         chars.push(c);
//                     }
//                 }
//                 chars.len() == s.len()
//             })
//             .collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::partition("fffffffff".to_string()));
        assert_eq!(1, 4);
    }
}
