use std::{collections::HashMap, ptr::NonNull};

use tadm::dlist::{DList, Link, Node};

#[derive(Copy, Clone)]
pub struct Element {
    key: i32,
    val: i32,
}

pub struct LRUCache {
    cap: usize,
    store: HashMap<i32, Link<Element>>,
    list: DList<Element>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        LRUCache {
            cap,
            store: HashMap::with_capacity(cap * 2),
            list: DList::new(),
        }
    }

    pub fn put(&mut self, key: i32, val: i32) {
        unsafe {
            let is_full = self.is_full();

            if is_full && self.len() == 0 {
                return;
            } else {
                if let Some(&link) = self.store.get(&key) {
                    (*link.as_ptr()).elem.assume_init_mut().val = val;
                    self.list.move_back_link(link);
                } else {
                    if is_full {
                        let front = self.list.pop_front().unwrap();
                        self.store.remove(&front.key);
                    }

                    let new_link =
                        NonNull::new(Box::into_raw(Box::new(Node::new(Element { key, val }))))
                            .unwrap();
                    self.store.insert(key, new_link);
                    self.list.push_back_link(new_link);
                }
            }
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        unsafe {
            if let Some(&link) = self.store.get(&key) {
                self.list.move_back_link(link);

                (*link.as_ptr()).elem.assume_init().val
            } else {
                -1
            }
        }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_full(&self) -> bool {
        self.store.len() == self.cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let mut cache = LRUCache::new(3);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);

        assert_eq!(cache.get(4), 4);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(1), -1);

        cache.put(5, 5);
        assert_eq!(cache.get(1), -1);

        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), -1);
        assert_eq!(cache.get(5), 5);
    }

    #[test]
    fn test1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);

        assert_eq!(cache.get(1), 1);

        cache.put(3, 3);

        assert_eq!(cache.get(2), -1);

        cache.put(4, 4);

        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
