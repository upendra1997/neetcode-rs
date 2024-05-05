use std::ops::{Index, IndexMut};

struct Solution;

const SIZE: usize = 'z' as usize - 'a' as usize + 1;

#[derive(PartialEq, Eq)]
struct SmallcaseChars {
    data: [usize; SIZE],
}

impl SmallcaseChars {
    pub fn new() -> Self {
        SmallcaseChars { data: [0; SIZE] }
    }

    pub fn len(&self) -> usize {
        self.data.iter().sum()
    }
}

impl Index<char> for SmallcaseChars {
    type Output = usize;

    fn index(&self, index: char) -> &Self::Output {
        let idx = index as usize - 'a' as usize;
        self.data.get(idx).unwrap()
    }
}

impl IndexMut<char> for SmallcaseChars {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        let idx = index as usize - 'a' as usize;
        self.data.get_mut(idx).unwrap()
    }
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut hashmaps1 = SmallcaseChars::new();
        let mut hashmaps2 = SmallcaseChars::new();
        for i in s1.chars() {
            hashmaps1[i] += 1;
        }
        let s2: Vec<char> = s2.chars().collect();
        let mut start = 0;
        for s in &s2 {
            hashmaps2[*s] += 1;
            let remove = hashmaps2.len() as i32 - hashmaps1.len() as i32;
            for _i in 0..remove {
                hashmaps2[s2[start]] -= 1;
                start += 1;
            }
            if hashmaps1 == hashmaps2 {
                return true;
            }
        }
        hashmaps1 == hashmaps2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "";
        let s2 = "taoeh";
        let result = true;
        assert_eq!(
            result,
            Solution::check_inclusion(s1.to_string(), s2.to_string())
        );
    }
    #[test]
    fn it_works1() {
        let s1 = "ab";
        let s2 = "eidbaooo";
        let result = true;
        assert_eq!(
            result,
            Solution::check_inclusion(s1.to_string(), s2.to_string())
        );
    }
    #[test]
    fn it_works2() {
        let s1 = "ab";
        let s2 = "eidboaoo";
        let result = false;
        assert_eq!(
            result,
            Solution::check_inclusion(s1.to_string(), s2.to_string())
        );
    }
}
