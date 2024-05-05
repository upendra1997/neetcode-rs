struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by_key(&target, |arr| arr[0]) {
            Ok(_) => true,
            Err(index) => {
                if index == 0 {
                    false
                } else {
                    match matrix[index - 1].binary_search(&target) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
