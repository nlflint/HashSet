use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

trait HashableValue: Hash + Eq + Clone {}
impl<T> HashableValue for T where T: Hash + Eq + Clone {}

#[derive(Debug, Clone)]
struct Node<T: Hash + Eq + Clone> {
    next: Option<Box<Node<T>>>,
    value: T
}

struct NatesHashSet<T: Hash + Eq + Clone>{
    count: usize,
    values: Vec<Option<Node<T>>>
}

fn new_hashset<T: Hash + Eq + Clone>() {
    NatesHashSet::<T> {
        count: 0,
        values: vec![Option::None; 16]
    };
}

#[test]
fn make_hashset() {
    let hash_set = new_hashset::<String>();
    
}

