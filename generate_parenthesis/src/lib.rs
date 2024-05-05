use std::collections::HashSet;

struct Solution;

impl Solution {
    fn stack(str: String, open_quota: i32, close_quota: i32, stack_size: i32) -> Vec<String> {
        if stack_size < 0 {
            return vec![];
        }
        let mut result = vec![];
        if open_quota == 0 && close_quota == 0 {
            result.push(str);
            return result;
        }
        if open_quota > 0 {
            result.extend(Solution::stack(
                format!("{}(", str),
                open_quota - 1,
                close_quota,
                stack_size + 1,
            ));
        }
        if close_quota > 0 {
            result.extend(Solution::stack(
                format!("{})", str),
                open_quota,
                close_quota - 1,
                stack_size - 1,
            ));
        }
        result
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Solution::stack("".to_string(), n, n, 0)
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
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(3), vec!["(", ")"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(", ")"]);
    }
}
