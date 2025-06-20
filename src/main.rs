mod myhash;

use myhash::{HashMap};

fn main() {
    let mut hash = HashMap::new(100);

    hash.put("a", "b");

    println!("{}", hash.get(&"a").unwrap_or(&"Not Found"));
}
