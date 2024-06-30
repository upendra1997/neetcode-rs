use std::{
    collections::{HashSet, VecDeque},
    iter, usize,
};

struct Solution;

struct Sol {
    heights: Vec<Vec<i32>>,
}

impl Sol {
    fn visited_nodes(&self, start_nodes: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new(); // from_iter(start_nodes.clone());
        let mut queue = VecDeque::from_iter(start_nodes.clone());
        let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while queue.len() > 0 {
            // println!("element: {:?}", queue);
            let element = queue.pop_front().unwrap();
            if result.get(&element).is_some() {
                continue;
            }
            result.insert(element);
            let (x, y) = element;
            for (dx, dy) in deltas {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x < 0 || new_x as usize >= self.heights.len() {
                    continue;
                }
                if new_x < 0 || new_y as usize >= self.heights[0].len() {
                    continue;
                }
                let coord = (new_x, new_y);
                // println!("new_coord: {:?}", coord);
                if result.get(&coord).is_some() {
                    continue;
                }
                if self.heights[new_x as usize][new_y as usize]
                    < self.heights[x as usize][y as usize]
                {
                    continue;
                }
                queue.push_back(coord);
            }
        }
        // println!("result: {:?}", result);
        result
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let pacific = Sol {
            heights: heights.clone(),
        };
        let atlantic = Sol {
            heights: heights.clone(),
        };
        let rows = heights.len();
        let cols = heights[0].len();
        let mut p_ocean_u = iter::repeat(0)
            .zip(0..cols)
            .map(|(x, y)| (x as i32, y as i32))
            .collect::<Vec<_>>();
        let mut p_ocean_l = (0..rows)
            .zip(iter::repeat(0))
            .map(|(x, y)| (x as i32, y as i32))
            .collect::<Vec<_>>();
        let mut a_ocean_r = (0..rows)
            .zip(iter::repeat(cols - 1))
            .map(|(x, y)| (x as i32, y as i32))
            .collect::<Vec<_>>();
        let mut a_ocean_b = iter::repeat(rows - 1)
            .zip((0..cols))
            .map(|(x, y)| (x as i32, y as i32))
            .collect::<Vec<_>>();
        p_ocean_l.extend(p_ocean_u);
        // println!("{:?}", p_ocean_l);
        let p = HashSet::from_iter(p_ocean_l);
        let p_r = pacific.visited_nodes(p);
        a_ocean_r.extend(a_ocean_b);
        // println!("{:?}", a_ocean_r);
        let a = HashSet::from_iter(a_ocean_r);
        let a_r = atlantic.visited_nodes(a);
        let mut result = vec![];
        for p in p_r {
            if a_r.contains(&p) {
                result.push(vec![p.0, p.1]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        let heights = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        let result = Solution::pacific_atlantic(heights);
        assert_eq!(result, vec![vec![]]);
    }
    #[test]
    fn it_works() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let result = Solution::pacific_atlantic(heights);
        assert_eq!(result, vec![vec![]]);
    }
}
