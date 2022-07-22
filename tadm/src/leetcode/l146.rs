// https://leetcode.com/problems/lru-cache/

// use std::{collections::HashMap, fmt, ptr};

// type Link = *mut Node;

// #[derive(Copy, Clone)]
// struct Node {
//     key: i32,
//     val: i32,
//     prev: Link,
//     next: Link,
// }

// impl Node {
//     fn new(key: i32, val: i32) -> Self {
//         Node {
//             key,
//             val,
//             prev: ptr::null_mut(),
//             next: ptr::null_mut(),
//         }
//     }
// }

// struct LRUCache {
//     cap: usize,
//     store: HashMap<i32, Link>,
//     head: Link,
//     tail: Link,
// }

// impl LRUCache {
//     fn new(capacity: i32) -> Self {
//         let cap = capacity as usize;
//         LRUCache {
//             cap,
//             store: HashMap::with_capacity(cap * 2),
//             head: ptr::null_mut(),
//             tail: ptr::null_mut(),
//         }
//     }

//     fn remove_link(&mut self, link: Link) {
//         unsafe {
//             if self.head == self.tail {
//                 self.head = ptr::null_mut();
//                 self.tail = ptr::null_mut();
//             } else if link == self.head {
//                 let node = (*link);
//                 self.head = node.next;
//                 let mut next_node = *node.next;
//                 next_node.prev = ptr::null_mut();
//             } else if link == self.tail {
//                 self.tail = (*link).prev;
//                 (*(*link).prev).next = ptr::null_mut();
//             } else {
//                 (*(*link).prev).next = (*link).next;
//                 (*(*link).next).prev = (*link).prev;
//             }
//         }
//     }

//     fn push_link(&mut self, link: Link) {
//         unsafe {
//             if self.tail.is_null() {
//                 self.head = link;
//                 self.tail = link;
//             } else {
//                 (*self.tail).next = link;
//                 (*link).prev = self.tail;
//                 self.tail = link;
//             }
//         }
//     }

//     fn put(&mut self, key: i32, value: i32) {
//         unsafe {
//             let is_full = self.store.len() == self.cap;

//             if (is_full && self.head.is_null()) {
//                 return;
//             } else {
//                 if let Some(&link) = self.store.get(&key) {
//                     (*link).val = value;
//                     self.remove_link(link);
//                     self.push_link(link);
//                 } else {
//                     if is_full {
//                         let lru_link = self.head;
//                         self.store.remove(&(*lru_link).key);
//                         self.remove_link(lru_link);
//                     }

//                     let new_node = Node::new(key, value);
//                     let new_link = Box::into_raw(Box::new(new_node)) as Link;
//                     self.store.insert(key, new_link);
//                     self.push_link(new_link);
//                 }
//             }
//         }
//     }

//     fn get(&mut self, key: i32) -> i32 {
//         unsafe {
//             let mut result;
//             if let Some(&link) = self.store.get(&key) {
//                 result = (*link).val;
//                 self.remove_link(link);
//                 self.push_link(link);
//             } else {
//                 result = -1
//             }

//             result
//         }
//     }
// }

use std::collections::HashMap;
use std::ptr;

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    data: HashMap<i32, Box<Node<i32, i32>>>, // use box to keep same memory address (for rehash)
    cache: LinkedList<i32, i32>,
    count: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            capacity,
            data: HashMap::with_capacity(capacity), //new(),
            cache: LinkedList::new(),
            count: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get_mut(&key) {
            self.cache.update_node(node.as_mut_ptr());
            node.val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            self.cache.update_node(node.as_mut_ptr());
            node.val = value;
        } else {
            if self.count == self.capacity {
                self.data.remove(self.cache.remove_head());
            } else {
                self.count += 1;
            }
            let val = Box::new(Node::new(key, value));
            let p = val.as_mut_ptr();
            self.data.insert(key, val);
            self.cache.push_back(p);
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K, // for fast pop from hashmap
    val: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const Self as usize as *mut Self
    }
}

/// no owned data
#[derive(Debug)]
pub struct LinkedList<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
}

impl<K, V> LinkedList<K, V> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn remove_node(&mut self, p: *mut Node<K, V>) -> &K {
        let node = unsafe { &mut *p };

        if self.head == self.tail {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else if p == self.head {
            self.head = node.next;
        } else if p == self.tail {
            self.tail = node.prev;
        } else {
            unsafe {
                (*node.prev).next = node.next;
                (*node.next).prev = node.prev;
            }
        }
        &node.key //.clone() //TODO: is it ok?
    }

    pub fn remove_head(&mut self) -> &K {
        self.remove_node(self.head)
    }

    pub fn push_back(&mut self, p: *mut Node<K, V>) {
        if self.head.is_null() {
            self.head = p;
            self.tail = p;
        } else {
            unsafe {
                (*self.tail).next = p;
                (*p).prev = self.tail;
                self.tail = p;
            }
        }
    }

    pub fn update_node(&mut self, p: *mut Node<K, V>) {
        if p != self.tail {
            self.remove_node(p);
            self.push_back(p);
        }
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
