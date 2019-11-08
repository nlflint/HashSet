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

struct HashSet<T: HashableValue>{
    values: Vec<Option<Vec<T>>>
}



impl<T> HashSet<T> where T: HashableValue {
    fn new() -> Self {
        HashSet::<T> {
            values: vec![Option::None; 16]
        }
    }

    fn add(self: &mut Self, value: T) -> bool {
        let hash_index = self.get_hash(&value);

        if self.contains(&value) {
            return false;
        }

        match &mut self.values[hash_index] {
            Some(ref mut set) => {
                set.push(value);
            }
            None => {
                self.values[hash_index] = Option::Some(vec![value]);
            }
        }

        true
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

    fn contains(self: &mut Self, value: &T) -> bool {
        let hash_index = self.get_hash(value);

        match &self.values[hash_index] {
            Some(set) => {
                set.iter().any(|x| x == value)
            },
            None => false
        }
    }

    fn remove(self: &mut Self, value: &T) -> bool {
        let hash_index = self.get_hash(value);

        match &mut self.values[hash_index] {
            Some(ref mut set) => {
                for i in 0..set.len() {
                    if set[i] == *value {
                        set.remove(i);
                        return true;
                    }
                }
                false
            },
            None => false
        }
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

#[test]
fn remove_where_not_exists() {
    //"f" and "h" both hash to 5
    let mut hash_set = new_hashset::<String>();
    let result = hash_set.remove(&"f".to_string());

    assert!(result == false);
}

