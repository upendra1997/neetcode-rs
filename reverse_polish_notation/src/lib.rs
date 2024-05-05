struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<&str>) -> i32 {
        let mut stack = Vec::new();
        stack.reserve(tokens.len());
        for i in tokens.iter() {
            match *i {
                "+" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first + second);

                },
                "-" => {

                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first * second);
                },
                "*" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first * second);
                }
                "/" => {
                    let second = stack.pop().unwrap();
                    let first = stack.pop().unwrap();
                    stack.push(first / second);
                }
                _ => {
                    stack.push(i.parse::<i32>().unwrap());
                }
            }
        }
        return stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::eval_rpn(vec!["2","1","+","3","*"]);
        assert_eq!(result, 9);
    }



    #[test]
    fn it_works_1() {
        let result = Solution::eval_rpn(vec!["4","13","5","/","+"]);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::eval_rpn(vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]);
        assert_eq!(result, 22);
    }
}
