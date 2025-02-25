use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_CAPACITY: usize = 8;
const LOAD_FACTOR: f64 = 0.75; // Resize when 75% full

#[derive(Debug)]
struct HashMap<K, V> {
    buckets: Vec<Option<(K, V, u64)>>, // (Key, Value, Hash)
    capacity: usize,
    len: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> HashMap<K, V> {
    // Constructor
    fn new() -> Self {
        Self {
            buckets: vec![None; INITIAL_CAPACITY],
            capacity: INITIAL_CAPACITY,
            len: 0,
        }
    }

    // Hash function
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    // Insert function
    fn insert(&mut self, key: K, value: V) {
        if self.len as f64 / self.capacity as f64 > LOAD_FACTOR {
            self.resize();
        }

        let hash = self.hash(&key);
        let mut index = (hash as usize) % self.capacity;
        let mut i = 0;

        // Quadratic probing
        while let Some((existing_key, _, _)) = &self.buckets[index] {
            if existing_key == &key {
                // Update existing key
                self.buckets[index] = Some((key, value, hash));
                return;
            }
            i += 1;
            index = (index + i * i) % self.capacity; // Quadratic probing
        }

        // Insert new key-value pair
        self.buckets[index] = Some((key, value, hash));
        self.len += 1;
    }

    // Lookup function
    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let mut index = (hash as usize) % self.capacity;
        let mut i = 0;

        // Quadratic probing search
        while let Some((existing_key, value, existing_hash)) = &self.buckets[index] {
            if existing_key == key && *existing_hash == hash {
                return Some(value);
            }
            i += 1;
            index = (index + i * i) % self.capacity;
        }

        None
    }

    // Deletion function
    fn remove(&mut self, key: &K) -> bool {
        let hash = self.hash(key);
        let mut index = (hash as usize) % self.capacity;
        let mut i = 0;

        // Quadratic probing search
        while let Some((existing_key, _, _)) = &self.buckets[index] {
            if existing_key == key {
                self.buckets[index] = None; // Mark as deleted (tombstone)
                self.len -= 1;
                return true;
            }
            i += 1;
            index = (index + i * i) % self.capacity;
        }

        false
    }

    // Resize function (doubling capacity)
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let old_buckets = std::mem::replace(&mut self.buckets, vec![None; new_capacity]);

        self.capacity = new_capacity;
        self.len = 0; // Reset count and re-insert items

        for bucket in old_buckets.into_iter().flatten() {
            let (key, value, _) = bucket;
            self.insert(key, value); // Re-insert with new capacity
        }
    }
}

// Example usage
fn main() {
    let mut map = HashMap::new();

    map.insert("apple", 5);
    map.insert("banana", 3);
    map.insert("carrot", 7);

    println!("{:?}", map.get(&"banana")); // Some(3)
    println!("{:?}", map.get(&"grape")); // None

    map.remove(&"banana");
    println!("{:?}", map.get(&"banana")); // None

    println!("{:?}", map); // Print the entire HashMap
}
