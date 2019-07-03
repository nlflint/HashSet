use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

trait HashableValue: Hash + Eq + Clone {}
impl<T> HashableValue for T where T: Hash + Eq + Clone {}

#[derive(Debug, Clone)]
struct Node<T: HashableValue> {
    next: Option<Box<Node<T>>>,
    value: T
}

struct HashSet<T: HashableValue>{
    count: usize,
    values: Vec<Option<Node<T>>>
}


impl<T> HashSet<T> where T: HashableValue {
    fn new() -> Self {
        HashSet::<T> {
            count: 0,
            values: vec![Option::None; 16]
        }
        
    }
    
    fn add(self: &mut Self, value: T) -> bool {
        return false;
    }
}

fn new_hashset<T: HashableValue>() -> HashSet::<T> {
    return HashSet::<T>::new();
}

#[test]
fn make_hashset() {
    let mut hash_set = new_hashset::<String>();
}

#[test]
fn fail_to_add() {
    let mut hash_set = new_hashset::<String>();
    let result = hash_set.add("yo".to_string());

    assert!(result == false);
    
}

