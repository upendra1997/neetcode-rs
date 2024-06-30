use std::{sync::mpsc, thread};

struct Solution;

struct Solution2 {
    vec: Vec<usize>,
    sender: mpsc::Sender<Vec<usize>>,
}

impl Solution2 {
    fn new(vec: usize, sender: mpsc::Sender<Vec<usize>>) -> Self {
        let mut vec = Vec::with_capacity(vec);
        for i in 0..vec.capacity() {
            vec.push(0);
        }
        Self { vec, sender }
    }

    fn valid(&self, end: usize) -> Vec<usize> {
        if end == 0 {
            return (0..self.vec.len()).collect();
        }
        let mut result = Vec::with_capacity(self.vec.len());
        'outer: for i in 0..self.vec.len() {
            for j in 0..end {
                let current = self.vec[j];
                if current == i {
                    continue 'outer;
                }
                if end.abs_diff(j) == current.abs_diff(i) {
                    continue 'outer;
                }
            }
            result.push(i);
        }
        result
    }

    fn search(&mut self, start: usize) {
        if start == self.vec.len() {
            // println!("end: {:?}", self.vec);
            self.sender.send(self.vec.clone()).unwrap();
            return;
        }
        for v in self.valid(start) {
            self.vec[start] = v;
            // println!("{:?}", self.vec);
            self.search(start + 1);
        }
        return;
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let (sender, reciever) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut res = Solution2::new(n.clone() as usize, sender.clone());
            res.search(0);
            drop(sender);
        });
        let mut result = vec![];
        for i in reciever.iter() {
            let mut r = Vec::with_capacity(n as usize);
            for j in i.iter() {
                let mut res = Vec::new();
                res.resize(n as usize, '.');
                res[*j] = 'Q';
                r.push(res.into_iter().collect::<String>());
            }
            result.push(r);
        }
        handle.join().unwrap();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::solve_n_queens(4);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }
}
