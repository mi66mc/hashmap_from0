mod myhash;

use myhash::{HashMap};

fn main() {
    let mut map = HashMap::new(16);

    map.put("name", "John");
    map.put("age", "25");

    println!("{:?}", map.get(&"name"));    // Some("John")
    println!("{:?}", map.get(&"age"));     // Some("25")

    map.put("name", "Carlos");
    println!("{:?}", map.get(&"name"));    // Some("Carlos")

    println!("{}", map.remove(&"age"));    // true
    println!("{:?}", map.get(&"age"));     // None
}
