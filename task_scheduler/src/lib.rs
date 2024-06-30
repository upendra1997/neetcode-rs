use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    usize,
};

struct Solution;

#[derive(Debug)]
struct Task {
    name: char,
    size: usize,
    time: usize,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.size.eq(&other.size)
    }
}

impl Eq for Task {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = HashMap::with_capacity(n as usize);
        let mut priority = BinaryHeap::new();
        let mut l = tasks.len();
        for i in tasks {
            *freq.entry(i).or_insert(0) += 1
        }
        for (k, v) in freq.into_iter() {
            priority.push(Task {
                name: k,
                size: v,
                time: 0,
            })
        }
        let mut tick = 0;
        let mut queue: VecDeque<Task> = VecDeque::new();
        while l > 0 {
            println!("{} {:?} {:?}", tick, priority, queue);
            if queue.front().is_some() && (*queue.front().unwrap()).time < tick {
                priority.push(queue.pop_front().unwrap())
            }
            match priority.pop() {
                Some(mut item) => {
                    item.size -= 1;
                    l -= 1;
                    println!("processing: {}", item.name);
                    if item.size > 0 {
                        item.time = tick + n as usize;
                        queue.push_back(item)
                    }
                }
                None => {
                    println!("idle");
                }
            }
            tick += 1;
        }
        tick as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tasks = vec![
            'A', 'A', 'A', 'B', 'B', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        ];
        let n = 7;
        let result = Solution::least_interval(tasks, n);
        assert_eq!(result, 8);
    }
}
