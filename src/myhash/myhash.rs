use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


#[derive(Debug, Clone)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> Pair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Pair { key, value }
    }
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<Pair<K, V>>>,
    size: usize,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(Vec::new());
        }

        HashMap {
            buckets,
            size
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.size as u64) as usize
    }

    pub fn put(&mut self, key: K, value: V) {
        let hashed = self.hash(&key);
        let bucket = &mut self.buckets[hashed];

        for pair in bucket.iter_mut() {
            if pair.key == key {
                pair.value = value;
                return;
            }
        }

        bucket.push(Pair::new(key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hashed = self.hash(key);
        let bucket = &self.buckets[hashed];

        for pair in bucket {
            if &pair.key == key {
                return Some(&pair.value);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> bool {
        let hashed = self.hash(key);
        let bucket = &mut self.buckets[hashed];

        if let Some(p) = bucket.iter().position(|pair| &pair.key == key)  {
            bucket.remove(p);
            return true;
        }

        false
    }
}