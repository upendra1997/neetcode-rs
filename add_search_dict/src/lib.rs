struct Solution;

use std::collections::{HashMap, HashSet};

const SIZE: usize = (('z' as u8 - 'a' as u8) + 1) as usize;
const DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct WordDictionary {
    array: HashMap<char, Box<WordDictionary>>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            array: HashMap::new(),
            end: false,
        }
    }

    fn add_word(&mut self, word: String) {
        if word.len() == 0 {
            self.end = true;
            return;
        }
        let (val, word) = word.split_at(1);
        let char = val.chars().next().unwrap();
        self.array
            .entry(char)
            .or_insert_with(|| Box::new(WordDictionary::new()))
            .add_word(word.to_string());
    }

    fn search(&self, word: String) -> bool {
        if word.len() == 0 {
            return self.end;
        }
        let (val, word) = word.split_at(1);
        let char = val.chars().next().unwrap();
        match char {
            '.' => {
                let mut res = false;
                for (_k, v) in self.array.iter() {
                    res = res || v.search(word.to_string())
                }
                res
            }
            _ => match self.array.get(&char) {
                Some(r) => r.search(word.to_string()),
                None => false,
            },
        }
    }

    fn count(self: &Self) -> usize {
        let mut res = if self.end { 1 } else { 0 };
        for (_k, v) in self.array.iter() {
            res += v.count();
        }
        res
    }

    fn remove(self: &mut Self, word: String) {
        if word.len() == 0 {
            self.end = false;
            return;
        }
        let (val, word) = word.split_at(1);
        let char = val.chars().next().unwrap();
        match self.array.get_mut(&char) {
            Some(sub) => {
                sub.remove(word.to_string());
                if sub.count() == 0 {
                    self.array.remove(&char).unwrap();
                }
            }
            None => {}
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[derive(Debug)]
struct DFS<'a> {
    visited: HashSet<(usize, usize)>,
    board: &'a Vec<Vec<char>>,
    result: HashSet<String>,
}

impl<'a> DFS<'a> {
    fn new(board: &'a Vec<Vec<char>>) -> Self {
        DFS {
            visited: HashSet::new(),
            board: board,
            result: HashSet::new(),
        }
    }

    fn start<'b>(
        self: &mut Self,
        pos: (usize, usize),
        trie: &'b mut Box<WordDictionary>,
        mut s: String,
    ) {
        println!("------------",);
        println!("result: {:?}", self.result);
        println!("pos: {:?}", pos);
        println!("trie: {:?}", trie);
        println!("s: {:?}", s);
        println!("------------",);
        if self.visited.get(&pos).is_some() {
            return;
        }
        let res = self
            .board
            .get(pos.0)
            .iter()
            .flat_map(|r| r.get(pos.1))
            .next();
        if res.is_none() {
            return;
        }
        let res = res.unwrap();
        let new_trie = trie.array.remove(res);
        if new_trie.is_none() {
            return;
        }
        let mut new_trie = new_trie.unwrap();
        s.push(*res);
        if new_trie.end {
            self.result.insert(s.clone());
            trie.remove(res.to_string())
        }
        self.visited.insert(pos);
        for (dx, dy) in DELTA {
            let new_pos_x = pos.0 as i32 + dx;
            let new_pos_y = pos.1 as i32 + dy;
            if new_pos_x >= 0 && new_pos_y >= 0 {
                self.start(
                    (new_pos_x as usize, new_pos_y as usize),
                    &mut new_trie,
                    s.clone(),
                );
            }
        }
        self.visited.remove(&pos);
        trie.array.insert(*res, new_trie);
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Box::new(WordDictionary::new());
        for w in words {
            trie.add_word(w);
        }
        println!("TRIE: {:?}", trie);
        let mut trie = Box::new(trie);
        let mut res = DFS::new(&board);
        for (r, row) in board.iter().enumerate() {
            for (c, _col) in row.iter().enumerate() {
                println!("starting new DFS at {:?}", (r, c));
                res.start((r, c), &mut trie, "".to_string());
            }
        }
        res.result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = WordDictionary::new();
        obj.add_word("b".to_string());
        obj.add_word("ba".to_string());
        obj.add_word("bab".to_string());
        obj.add_word("bb".to_string());
        obj.add_word("bba".to_string());
        obj.remove("b".to_string());
        obj.remove("bb".to_string());
        obj.remove("ba".to_string());
        obj.remove("bab".to_string());
        assert!(obj.search("bba".to_string()));
        assert_eq!(obj.count(), 1);
    }

    #[test]
    fn test2() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string(),
        ];
        let result = Solution::find_words(board, words);
        assert_eq!(
            result.into_iter().collect::<HashSet<String>>(),
            ["oath".to_string(), "eat".to_string()]
                .into_iter()
                .collect::<HashSet<String>>()
        )
    }

    #[test]
    fn test3() {
        let board = vec![vec!['b', 'a'], vec!['b', 'b']];
        let words = vec![
            "bb".to_string(),
            "bab".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bba".to_string(),
        ];
        let result = Solution::find_words(board, words);
        assert_eq!(
            result.into_iter().collect::<HashSet<String>>(),
            [
                "b".to_string(),
                "ba".to_string(),
                "bab".to_string(),
                "bb".to_string(),
                "bba".to_string()
            ]
            .into_iter()
            .collect::<HashSet<String>>()
        )
    }
}
