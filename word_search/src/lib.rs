use std::{collections::HashSet, usize};

#[derive(Debug, PartialEq)]
struct DFS {
    visited: HashSet<(usize, usize)>,
    grid: Vec<Vec<char>>,
    needle: String,
    status: bool,
}

impl DFS {
    fn new(grid: Vec<Vec<char>>, needle: String) -> Self {
        let visited = HashSet::with_capacity(grid.len() * grid[0].len());
        DFS {
            grid,
            needle,
            visited,
            status: false,
        }
    }

    fn search(self: &mut Self, start: (usize, usize)) {
        if self.status {
            return;
        }
        if self.needle.len() == 0 {
            self.status = true;
            return;
        }
        if self.visited.get(&start).is_some() {
            return;
        }
        let c = self.needle.chars().next().unwrap();
        if self.grid[start.0][start.1] != c {
            return;
        }
        self.needle = self.needle.chars().skip(1).collect();
        let mut delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        self.visited.insert(start);
        for (x, y) in delta {
            let new_x = start.0 as isize + x;
            let new_y = start.1 as isize + y;
            if new_x < 0 || new_x >= self.grid.len() as isize {
                continue;
            }
            if new_y < 0 || new_y >= self.grid[0].len() as isize {
                continue;
            }
            self.search((new_x as usize, new_y as usize));
        }
        self.needle.insert(0, c);
        self.visited.remove(&start);
    }
}

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 1 {
            let c = word.chars().next().unwrap();
            for (r, row) in board.iter().enumerate() {
                for (_, chr) in row.iter().enumerate() {
                    if c == *chr {
                        return true;
                    }
                }
            }
        }
        for (r, row) in board.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                let mut result = DFS::new(board.clone(), word.clone());
                result.search((r, c));
                if result.status {
                    return true;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['C', 'A', 'A'],
            vec!['A', 'A', 'A'],
            vec!['B', 'C', 'D'],
        ];
        let mut result = DFS::new(board.clone(), "AAB".to_string());
        result.search((1, 1));
        assert_eq!(result, DFS::new(vec![vec![]], "".to_string()));
    }
    #[test]
    fn it_works_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let mut result = DFS::new(board.clone(), "ABCB".to_string());
        result.search((0, 0));
        // assert_eq!(result.visited, [].into());
        assert_eq!(result, DFS::new(vec![vec![]], "".to_string()));
    }
    #[test]
    fn it_works_1() {
        let board = vec![vec!['A', 'B'], vec!['C', 'D']];
        let mut result = DFS::new(board.clone(), "CDBA".to_string());
        result.search((1, 0));
        assert_eq!(result, DFS::new(vec![vec![]], "".to_string()));
    }
}
