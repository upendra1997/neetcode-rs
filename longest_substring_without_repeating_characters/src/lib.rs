struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hashmap: Vec<i32> = std::iter::repeat(-1).take(u8::MAX as usize + 1).collect();
        let mut start = 0;
        let mut maxi = 1;
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let s = s.into_bytes();
        for (idx, i) in s.iter().enumerate() {
            let i = *i as usize;
            if hashmap[i] != -1 {
                maxi = maxi.max(idx - start);
                let temp = hashmap[i] as usize;
                println!("{}:{} = {}", idx, i, start);
                while start <= temp {
                    hashmap[s[start] as usize] = -1;
                    start += 1;
                }
            }
            hashmap[i] = idx as i32;
        }
        maxi = maxi.max(n - start);
        maxi as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = "abcabcbb";
        let result = 3;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }
    #[test]
    fn it_works1() {
        let inp = "bbbbb";
        let result = 1;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }
    #[test]
    fn it_works2() {
        let inp = "pwwkew";
        let result = 3;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }

    #[test]
    fn it_works3() {
        let inp = " ";
        let result = 1;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }

    #[test]
    fn it_works4() {
        let inp = "tmmzuxt";
        let result = 5;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }

    #[test]
    fn it_works5() {
        let inp = "hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789hijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let result = 55;
        assert_eq!(
            result,
            Solution::length_of_longest_substring(inp.to_string())
        );
    }
}
