struct Solution {}

impl Solution {
    pub fn max_area(mut height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;
        while left < right {
            res =
                (height[left].min(height[right]) * ((right - left) as i32)).max(res);
            match height[left].cmp(&height[right]) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    left += 1;
                },
                std::cmp::Ordering::Greater => {
                    right -= 1;
                },
            }
        }
        return res;
    }
}

#[test]
fn test1() {
    let res = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    assert_eq!(res, 49);
}

#[test]
fn test2() {
    let res = Solution::max_area(vec![1, 1]);
    assert_eq!(res, 1);
}
