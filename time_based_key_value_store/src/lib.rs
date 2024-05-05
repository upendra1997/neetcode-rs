use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let mut vec = self.store.entry(key).or_default();
        vec.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let empty = vec![];
        let entry = self.store.get(&key).unwrap_or(&empty);
        match entry.binary_search_by_key(&timestamp, |v| v.0) {
            Ok(result) => entry[result].1.clone(),
            Err(result) => {
                if result == 0 {
                    "".to_string()
                } else {
                    entry[result - 1].1.clone()
                }
            }
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
