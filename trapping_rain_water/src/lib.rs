struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_right_idx = height.len() - 1;
        let mut max_left_idx: usize = 0;
        let mut max_from_right = height[max_right_idx];
        let mut max_from_left  = height[max_left_idx];
        let mut res = 0;
        let mut idx = max_left_idx;
        while max_left_idx < max_right_idx {
            res += 0.max(max_from_left.min(max_from_right) - height[idx]);
            if max_from_left < max_from_right {
                max_left_idx += 1;
                max_from_left = height[max_left_idx].max(max_from_left);
                idx = max_left_idx;
            } else {
                max_right_idx -= 1;
                max_from_right = height[max_right_idx].max(max_from_right);
                idx = max_right_idx;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::trap(vec![4,2,0,3,2,5]);
        assert_eq!(result, 9);
    }

    #[test]
    fn test2() {
        let result = Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
        assert_eq!(result, 6);
    }
}



// [0,1,0,2,1,0,1,3,2,1,2,1] = A
// [3,3,3,3,3,3,3,3,2,2,2,1] = B
// [0,1,1,2,2,2,2,3,3,3,3,3] = C
// [0,0,1,0,1,2,1,0,0,1,0,0] = R = max(0, (min(B,C) - A))