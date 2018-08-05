trait Hashable {
    fn to_hash (&self) -> u32; 
}

trait Map <K, V> where K: Hashable, V: PartialEq { 
    fn put(&mut self, key: K, val: V);
    /// reference to the item in the map
    fn get(&self, key: K) -> Option<&V>;
    fn get_mut(&mut self, key: K) -> Option<&mut V>;
    fn remove(&mut self, key: K) -> Option<V>;
}

struct LinkedHashmap <K, V> where K: Hashable + Sized + PartialEq, V: Sized {
    bins: Vec<Vec<(K, V)>>,
    num_bins: u32,
}

impl<K, V> LinkedHashmap <K, V> where K: Hashable + PartialEq + Sized, V: Sized {
    fn new(bins: u32) -> Self {
        let mut map = LinkedHashmap {
            num_bins: bins as u32, 
            bins: Vec::with_capacity(bins as usize),
        };
        for _ in 0..bins {
            map.bins.push(Vec::new());
        }
        map
    }

    fn bin_key (&self, key: &K)  -> u32 {
        key.to_hash() % self.num_bins 
    }
}

use std::fmt::Display;
impl<K,V> Map<K, V> for LinkedHashmap <K, V> where K: Hashable + Sized + PartialEq + Display, V: Sized + PartialEq {

    fn put(&mut self, key: K, val: V) {
        let bin_key = self.bin_key(&key); 
        let ref mut bin = self.bins.get_mut(bin_key as usize).expect(format!("couldn't find bin {}", bin_key).as_str());
        let mut already_contains = false;
        for (ref k, ref v) in bin.iter() {
            if k == &key {
                already_contains = true;
            }
        }
        if already_contains {
            println!("Item already in hashmap!");
        }
        else {
            bin.push((key, val));
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        let bin_key = self.bin_key(&key);
        let ref bin = self.bins.get(bin_key as usize).expect(format!("couldn't find bin {}", bin_key).as_str());
        let mut ret_val: Option<&V> = None;
        for (ref k, ref v) in bin.iter() {
            if k == &key {
                ret_val = Some(v)
            }
        }
        ret_val
    }
    
    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let bin_key = self.bin_key(&key);
        let bin = self.bins.get_mut(bin_key as usize).expect(format!("couldn't find bin {}", bin_key).as_str());
        let mut ret_val: Option<&mut V> = None;
        for (ref k, ref mut v) in bin.iter_mut() {
            if k == &key {
                ret_val = Some(v)
            }
        }
        ret_val
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let bin_key = self.bin_key(&key);
        let bin = self.bins.get_mut(bin_key as usize).expect(format!("couldn't find bin {}", bin_key).as_str());
        let mut i: i32 = -1;
        for (ref k, ref v) in bin.iter() {
            i += 1;
            if k == &key {
                break; 
            }
        }
        if i == -1 {
            None
        } 
        else {
            let (_, v) = bin.remove(i as usize);
            Some(v)
        }
    }
}

use std::hash::{Hash, Hasher};
use std::collections::hash_map::{DefaultHasher}; 
impl Hashable for String {
    fn to_hash(&self) -> u32{
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as u32
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn make_map() {
        let map = LinkedHashmap::<String, String>::new(5);
    }
    #[test]
    fn put() {
        let mut map = LinkedHashmap::<String, String>::new(5);
        map.put("a".to_string(), "b".to_string());
    }

    #[test]
    fn get() {
        let mut map = LinkedHashmap::<String, String>::new(5);
        map.put("key1".to_string(), "val1".to_string());
        assert_eq!(&"val1".to_string(), map.get("key1".to_string()).expect("get failed"));
    }
    #[test]
    fn get_mut() {
        let mut map = LinkedHashmap::<String, String>::new(5);
        map.put("key1".to_string(), "val1".to_string());
        map.get_mut("key1".to_string()).unwrap().push('d');
        assert_eq!(&"val1d".to_string(), map.get("key1".to_string()).expect("get failed"));
    }

    #[test]
    fn remove() {
        let mut map = LinkedHashmap::<String, String>::new(5);
        map.put("key1".to_string(), "val1".to_string());
        map.remove("key1".to_string());
        assert_eq!(None, map.get("key1".to_string()));
    }
    #[test]
    fn collide() {
        let mut map = LinkedHashmap::<String, String>::new(1);
        map.put("key1".to_string(), "val1".to_string());
        map.put("key2".to_string(), "val2".to_string());
        map.remove("key1".to_string());
        assert_eq!(&"val2".to_string(), map.get("key2".to_string()).expect("failed to get key 2"));
    }
}
