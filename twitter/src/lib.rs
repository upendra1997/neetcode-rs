use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const N: usize = 10;

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Tweet(usize, i32);
#[derive(Debug, Eq, Hash, PartialEq)]
struct User(i32);

#[derive(Debug)]
struct Twitter {
    followers: HashMap<User, HashSet<User>>,
    tweets: HashMap<User, VecDeque<Tweet>>,
    tick: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            followers: HashMap::new(),
            tweets: HashMap::new(),
            tick: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let mut heap = self
            .tweets
            .entry(User(user_id))
            .or_insert(VecDeque::with_capacity(N));
        let tweet = Tweet(self.tick, tweet_id);
        self.tick += 1;
        if heap.len() == N {
            heap.pop_front();
        }
        heap.push_back(tweet);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut empty = HashSet::new();
        empty.insert(User(user_id));
        println!("{:?}", self);
        let followers = self.followers.get(&User(user_id)).unwrap_or(&empty);
        let mut result = vec![];
        for i in followers.iter() {
            if let Some(tweets) = self.tweets.get(i) {
                result.extend(tweets);
            }
        }
        let mut heap = BinaryHeap::from(result);
        println!("{:?}", heap);
        let mut result = Vec::with_capacity(N);
        while result.len() < N {
            match heap.pop() {
                Some(r) => result.push(r.1),
                None => break,
            }
        }
        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let mut set = self
            .followers
            .entry(User(follower_id))
            .or_insert(HashSet::new());
        set.insert(User(followee_id));
        set.insert(User(follower_id));
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        let mut set = self
            .followers
            .entry(User(follower_id))
            .or_insert(HashSet::new());
        set.insert(User(follower_id));
        set.remove(&User(followee_id));
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 4); // User 1 posts a new tweet (id = 5).
        twitter.post_tweet(2, 5); // User 1 posts a new tweet (id = 5).
        twitter.unfollow(1, 2); // User 1 unfollows user 2.
        println!("{:?}", twitter.get_news_feed(1)); // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.

        assert_eq!(0, 4);
    }
}
