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

fn get_hash<T: HashableValue>(value: T, limit: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish() as usize;
    hash % limit
}

impl<T> HashSet<T> where T: HashableValue {
    fn new() -> Self {
        HashSet::<T> {
            count: 0,
            values: vec![Option::None; 16]
        }
    }
    
    fn add(self: &mut Self, value: T) -> bool {
        let hash_index = get_hash(&value, self.capacity());

        match &self.values[hash_index]{
            Some(x) => {return false},
            None => {
                self.insert(value, hash_index);
                return true;
            }
        }
    }

    fn capacity(self: &mut Self) -> usize {
        return self.values.len();
    }

    fn insert(self: &mut Self, value: T, insert_index: usize) {
        let new_node = Node {
            next: Option::None,
            value: value
        };

        let new_value = Some(new_node);

        self.values[insert_index] = new_value;
    }

    fn contains(self: &mut Self, value: T) -> bool {
        true
    }
}

fn new_hashset<T: HashableValue>() -> HashSet::<T> {
    return HashSet::<T>::new();
}

#[test]
fn make_hashset() {
    let hash_set = new_hashset::<String>();
}

#[test]
fn add_value() {
    let mut hash_set = new_hashset::<String>();
    let result = hash_set.add("yo".to_string());
    let contains = hash_set.contains("yo".to_string());
    assert!(result == true);
    assert!(contains == true);

}



