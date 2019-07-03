use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Node<T: Hash + Eq + Clone> {
    next: Option<Box<Node<T>>>,
    value: T
}

struct HashSet<T: Hash + Eq + Clone>{
    count: usize,
    values: Vec<Option<Node<T>>>
}

fn new_hashset<T: Hash + Eq + Clone>() {
    HashSet::<T> {
        count: 0,
        values: vec![Option::None; 16]
    };
}



#[test]
fn make_hashset() {
    let hash_set = new_hashset::<String>();
    
}

