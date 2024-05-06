struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut output = false;
        let mut mini = s.len();
        let mut hashmap_t = [0; (u8::MAX as usize + 1)];
        let mut hashmap_s = [0; (u8::MAX as usize + 1)];
        for i in t.bytes() {
            hashmap_t[i as usize] += 1;
        }
        let s: Vec<u8> = s.bytes().collect();
        let mut start = 0;
        let mut min_start = 0;
        for (idx, i) in s.iter().enumerate() {
            hashmap_s[*i as usize] += 1;
            let result = |hashmap_s: [i32; 256], hashmap_t: [i32; 256]| {
                let mut b = false;
                for (t, s) in hashmap_t.iter().zip(hashmap_s.iter()) {
                    if s >= t {
                    } else {
                        b = true;
                        break;
                    }
                }
                if b {
                    false
                } else {
                    true
                }
            };
            while result(hashmap_s, hashmap_t) {
                output = true;
                println!(
                    "{}",
                    String::from_utf8_lossy(&s[start..idx + 1]).to_string()
                );
                if mini > idx - start + 1 {
                    mini = idx - start + 1;
                    min_start = start;
                }
                hashmap_s[s[start] as usize] -= 1;
                start += 1;
            }
        }
        if output {
            String::from_utf8_lossy(&s[min_start..min_start + mini]).to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "";
        let t = "";
        let result = "";
        let solution = Solution::min_window(s.to_string(), t.to_string());
        assert_eq!(result, solution);
    }
    #[test]
    fn it_works1() {
        let s = "ADOBECODEBANC";
        let t = "ABC";
        let result = "BANC";
        let solution = Solution::min_window(s.to_string(), t.to_string());
        assert_eq!(result, solution);
    }
    #[test]
    fn it_works2() {
        let s = "a";
        let t = "a";
        let result = "a";
        let solution = Solution::min_window(s.to_string(), t.to_string());
        assert_eq!(result, solution);
    }
    #[test]
    fn it_works3() {
        let s = "a";
        let t = "aa";
        let result = "";
        let solution = Solution::min_window(s.to_string(), t.to_string());
        assert_eq!(result, solution);
    }
}
