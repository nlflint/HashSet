use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

fn main() {
    let mut hash_set = new_hashset::<String>();
    
    let hash = hash_set.get_hash(&"a".to_string());
    println!("a: {}", hash);

    let hash = hash_set.get_hash(&"b".to_string());
    println!("b: {}", hash);

    let hash = hash_set.get_hash(&"c".to_string());
    println!("c: {}", hash);

    let hash = hash_set.get_hash(&"d".to_string());
    println!("d: {}", hash);

    let hash = hash_set.get_hash(&"e".to_string());
    println!("e: {}", hash);

    let hash = hash_set.get_hash(&"f".to_string());
    println!("f: {}", hash);

    let hash = hash_set.get_hash(&"g".to_string());
    println!("g: {}", hash);

    let hash = hash_set.get_hash(&"h".to_string());
    println!("h: {}", hash);
}

trait HashableValue: Hash + Eq + Clone {}
impl<T> HashableValue for T where T: Hash + Eq + Clone {}

#[derive(Debug, Clone)]
struct Node<T: HashableValue> {
    next: Box<Option<Node<T>>>,
    value: T
}

struct HashSet<T: HashableValue>{
    //count: usize,
    values: Vec<Option<Node<T>>>
}



impl<T> HashSet<T> where T: HashableValue {
    fn new() -> Self {
        HashSet::<T> {
            //count: 0,
            values: vec![Option::None; 16]
        }
    }
    
    fn add(self: &mut Self, value: T) -> bool {
        let hash_index = self.get_hash(&value);

        if self.contains(&value) {
            return false;
        }

        match &self.values[hash_index]{
            Some(_) => {
                let old_head = mem::replace(&mut self.values[hash_index], None);
                let new_head = Some
                (
                    Node{
                        value: value,
                        next: Box::new(old_head)
                    }
                    
                );
                mem::replace(&mut self.values[hash_index], new_head);
                return true;
            },
            None => {
                self.insert(value, hash_index);
                return true;
            }
        }
    }

    fn get_hash(self: &mut Self, value: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        hash % self.capacity()
    }

    fn capacity(self: &mut Self) -> usize {
        return self.values.len();
    }

    fn insert(self: &mut Self, value: T, insert_index: usize) {
        let new_node = Node {
            next: Box::new(Option::None),
            value: value
        };

        let new_value = Some(new_node);

        self.values[insert_index] = new_value;
    }

    fn contains(self: &mut Self, value: &T) -> bool {
        let hash_index = self.get_hash(value);
        
        let mut maybe_node = &self.values[hash_index];
        loop {
            match maybe_node {
                Some(node) => {
                    if &node.value == value {
                        return true;
                    } else {
                        maybe_node = &*node.next
                    }
                },
                None => {return false;}
            }
        }
    }

    fn remove(self: &mut Self, value: &T) -> bool {
        let hash_index = self.get_hash(value);
        let detached_head = mem::replace(&mut self.values[hash_index], None);

        match detached_head {
            Some(node) => {
                if &node.value == value {
                    mem::replace(&mut self.values[hash_index], *node.next);
                    return true;
                }
                mem::replace(&mut self.values[hash_index], Some(node));
            },
            None => {return false;}
        }

        // detached_head = mem::replace(&mut self.values[hash_index], None);
        // loop {
        //     match maybe_node {
        //         Some(node) => {
        //             let next_node = *node.next
        //             if &next_node.value == value {
        //                 node.nex
        //                 return true;
        //             } else {
        //                 maybe_node = &*node.next
        //             }
        //         },
        //         None => {return false;}
        //     }
        // }

        return false;
    }
}

fn new_hashset<T: HashableValue>() -> HashSet::<T> {
    return HashSet::<T>::new();
}

#[test]
fn does_not_contain() {
    let mut hash_set = new_hashset::<String>();
    let contains = hash_set.contains(&"yo".to_string());
    assert!(contains == false);
}

#[test]
fn add_value() {
    let mut hash_set = new_hashset::<String>();
    let result = hash_set.add("yo".to_string());
    let contains = hash_set.contains(&"yo".to_string());
    
    assert!(result == true);
    assert!(contains == true);
}

#[test]
fn does_not_contain_on_hash_collision() {
    //"f" and "h" both hash to 5
    let mut hash_set = new_hashset::<String>();
    hash_set.add("f".to_string());
    
    let contains = hash_set.contains(&"h".to_string());

    assert!(contains == false);

}

#[test]
fn contains_both_on_hash_collision() {
    //"f" and "h" both hash to 5
    let mut hash_set = new_hashset::<String>();
    hash_set.add("f".to_string());
    hash_set.add("h".to_string());
    
    let contains_f = hash_set.contains(&"f".to_string());
    let contains_h = hash_set.contains(&"h".to_string());
    
    assert!(contains_f == true);
    assert!(contains_h == true);
}

#[test]
fn remove() {
    let mut hash_set = new_hashset::<String>();
    hash_set.add("a".to_string());
    assert!(hash_set.contains(&"a".to_string()) == true);
        
    hash_set.remove(&"a".to_string());
    assert!(hash_set.contains(&"a".to_string()) == false);
}

#[test]
fn remove_head_on_duplciate_hash() {
    //"f" and "h" both hash to 5
    let mut hash_set = new_hashset::<String>();
    hash_set.add("f".to_string());
    hash_set.add("h".to_string());
    hash_set.remove(&"h".to_string());

    assert!(hash_set.contains(&"h".to_string()) == false);
    assert!(hash_set.contains(&"f".to_string()) == true);
}

#[test]
fn remove_middle_on_duplciate_hash() {
    //"f" and "h" both hash to 5
    let mut hash_set = new_hashset::<String>();
    hash_set.add("f".to_string());
    hash_set.add("h".to_string());
    hash_set.remove(&"f".to_string());

    assert!(hash_set.contains(&"f".to_string()) == false);
    assert!(hash_set.contains(&"h".to_string()) == true);
}


