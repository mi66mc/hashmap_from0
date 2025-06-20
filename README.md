# Building a HashMap from Scratch in Rust

This project is a simple and educational implementation of a `HashMap` (hash table) in Rust. It supports basic operations like insertion (`put`), retrieval (`get`), and removal (`remove`), using **separate chaining** (with vectors) to handle collisions.

---

## 📁 Structure

- `Pair<K, V>`: Represents a key-value pair.
- `HashMap<K, V>`: Stores a vector of buckets (`Vec<Vec<Pair>>`) and implements core methods.

---

## 🚀 Features

- `put(key, value)` — Adds or updates a key-value pair.
- `get(&key)` — Retrieves the value for a given key (if present).
- `remove(&key)` — Deletes a key from the map if exists.
- Uses `DefaultHasher` from Rust's standard library for hashing.

---

## 🛠️ Example Usage

```rust
fn main() {
    let mut map = HashMap::new(16);

    map.put("name", "John");
    map.put("age", "25");

    println!("{:?}", map.get(&"name"));    // Some("John")
    println!("{:?}", map.get(&"age"));     // Some("25")

    map.put("name", "Carlos");
    println!("{:?}", map.get(&"name"));    // Some("Carlos")

    map.remove(&"age");
    println!("{:?}", map.get(&"age"));     // None
}
```