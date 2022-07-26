use core::{fmt, mem::replace};

pub struct LRUCache<T> {
    /// The most-recently-used entry is at index `head`. The entries form a linked list, linked to
    /// each other by indices within the `entries` array.  After an entry is added to the array,
    /// its index never changes, so these links are never invalidated.
    entries: Vec<Entry<T>>,
    /// Index of the first entry. If the cache is empty, ignore this field.
    head: u16,
    /// Index of the last entry. If the cache is empty, ignore this field.
    tail: u16,
}

/// An entry in an `LRUCache`.
#[derive(Debug, Clone)]
struct Entry<T> {
    val: T,
    /// Index of the previous entry. If this entry is the head, ignore this field.
    prev: u16,
    /// Index of the next entry. If this entry is the tail, ignore this field.
    next: u16,
}

impl<T> Default for LRUCache<T> {
    fn default() -> Self {
        let cache = LRUCache {
            entries: Vec::new(),
            head: 0,
            tail: 0,
        };
        assert!(
            cache.entries.capacity() < u16::max_value() as usize,
            "Capacity overflow"
        );
        cache
    }
}

impl<T> LRUCache<T> {
    /// Insert a given key in the cache.
    ///
    /// This item becomes the front (most-recently-used) item in the cache.  If the cache is full,
    /// the back (least-recently-used) item will be removed and returned.
    pub fn insert(&mut self, val: T) -> Option<T> {
        let entry = Entry {
            val,
            prev: 0,
            next: 0,
        };

        // If the cache is full, replace the oldest entry. Otherwise, add an entry.
        let (new_head, previous_entry) = if self.entries.len() == self.entries.capacity() {
            let i = self.pop_back();
            let previous_entry = replace(&mut self.entries[i as usize], entry);
            (i, Some(previous_entry.val))
        } else {
            self.entries.push(entry);
            (self.entries.len() as u16 - 1, None)
        };

        self.push_front(new_head);
        previous_entry
    }

    /// Returns the first item in the cache that matches the given predicate.
    /// Touches the result (makes it most-recently-used) on a hit.
    pub fn find<F>(&mut self, pred: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> bool,
    {
        if self.touch(pred) {
            self.front_mut()
        } else {
            None
        }
    }

    /// Performs a lookup on the cache with the given test routine. Touches
    /// the result on a hit.
    pub fn lookup<F, R>(&mut self, mut test_one: F) -> Option<R>
    where
        F: FnMut(&mut T) -> Option<R>,
    {
        let mut result = None;
        let mut iter = self.iter_mut();
        while let Some((i, val)) = iter.next() {
            if let Some(r) = test_one(val) {
                result = Some((i, r));
                break;
            }
        }

        match result {
            None => None,
            Some((i, r)) => {
                self.touch_index(i);
                Some(r)
            }
        }
    }

    /// Returns the number of elements in the cache.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Evict all elements from the cache.
    #[inline]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the front entry in the list (most recently used).
    pub fn front(&self) -> Option<&T> {
        self.entries.get(self.head as usize).map(|e| &e.val)
    }

    /// Returns a mutable reference to the front entry in the list (most recently used).
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.entries.get_mut(self.head as usize).map(|e| &mut e.val)
    }

    /// Returns the n-th entry in the list (most recently used).
    pub fn get(&self, index: usize) -> Option<&T> {
        self.iter().nth(index)
    }

    /// Touches the first item in the cache that matches the given predicate (marks it as
    /// most-recently-used).
    /// Returns `true` on a hit, `false` if no matches.
    pub fn touch<F>(&mut self, mut pred: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        let mut iter = self.iter_mut();
        while let Some((i, val)) = iter.next() {
            if pred(val) {
                self.touch_index(i);
                return true;
            }
        }
        false
    }

    /// Iterate over the contents of this cache in order from most-recently-used to
    /// least-recently-used.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            pos: self.head,
            cache: self,
        }
    }

    /// Iterate mutably over the contents of this cache in order from most-recently-used to
    /// least-recently-used.
    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            pos: self.head,
            cache: self,
        }
    }

    /// Touch a given entry, putting it first in the list.
    #[inline]
    fn touch_index(&mut self, idx: u16) {
        if idx != self.head {
            self.remove(idx);
            self.push_front(idx);
        }
    }

    /// Remove an entry from the linked list.
    ///
    /// Note: This only unlinks the entry from the list; it does not remove it from the array.
    fn remove(&mut self, i: u16) {
        let prev = self.entries[i as usize].prev;
        let next = self.entries[i as usize].next;

        if i == self.head {
            self.head = next;
        } else {
            self.entries[prev as usize].next = next;
        }

        if i == self.tail {
            self.tail = prev;
        } else {
            self.entries[next as usize].prev = prev;
        }
    }

    /// Insert a new entry at the head of the list.
    fn push_front(&mut self, i: u16) {
        if self.entries.len() == 1 {
            self.tail = i;
        } else {
            self.entries[i as usize].next = self.head;
            self.entries[self.head as usize].prev = i;
        }
        self.head = i;
    }

    /// Remove the last entry from the linked list. Returns the index of the removed entry.
    ///
    /// Note: This only unlinks the entry from the list; it does not remove it from the array.
    fn pop_back(&mut self) -> u16 {
        let old_tail = self.tail;
        let new_tail = self.entries[old_tail as usize].prev;
        self.tail = new_tail;
        old_tail
    }
}

impl<T> Clone for LRUCache<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            head: self.head,
            tail: self.tail,
        }
    }
}

impl<T> fmt::Debug for LRUCache<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LRUCache")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("entries", &self.entries)
            .finish()
    }
}

/// Mutable iterator over values in an `LRUCache`, from most-recently-used to least-recently-used.
struct IterMut<'a, T> {
    cache: &'a mut LRUCache<T>,
    pos: u16,
}

impl<'a, T> IterMut<'a, T> {
    fn next(&mut self) -> Option<(u16, &mut T)> {
        let index = self.pos;
        let entry = self.cache.entries.get_mut(index as usize)?;

        self.pos = if index == self.cache.tail {
            3001
        } else {
            entry.next
        };
        Some((index, &mut entry.val))
    }
}

/// Iterator over values in an [`LRUCache`], from most-recently-used to least-recently-used.
pub struct Iter<'a, T> {
    cache: &'a LRUCache<T>,
    pos: u16,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let entry = self.cache.entries.get(self.pos as usize)?;

        self.pos = if self.pos == self.cache.tail {
            3001
        } else {
            entry.next
        };
        Some(&entry.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestCache = LRUCache<i32>;

    /// Convenience function for test assertions
    fn items<T>(cache: &mut LRUCache<T>) -> Vec<T>
    where
        T: Clone,
    {
        let mut v = Vec::new();
        let mut iter = cache.iter_mut();
        while let Some((_idx, val)) = iter.next() {
            v.push(val.clone())
        }
        v
    }

    #[test]
    fn empty() {
        let mut cache = TestCache::default();
        assert_eq!(cache.is_empty(), true);
        assert_eq!(items(&mut cache), []);
        cache.insert(1);
        assert_eq!(cache.is_empty(), false);
    }

    #[test]
    fn len() {
        let mut cache = TestCache::default();
        cache.insert(1);
        assert_eq!(cache.len(), 1);
        assert_eq!(items(&mut cache), [1]);
    }

    #[test]
    fn insert() {
        let mut cache = TestCache::default();
        cache.insert(1);
        assert_eq!(cache.len(), 1);
        cache.insert(2);
        assert_eq!(cache.len(), 2);
        cache.insert(3);
        assert_eq!(cache.len(), 3);
        cache.insert(4);
        assert_eq!(cache.len(), 4);
        assert_eq!(
            items(&mut cache),
            [4, 3, 2, 1],
            "Ordered from most- to least-recent."
        );

        cache.insert(5);
        assert_eq!(cache.len(), 4);
        assert_eq!(
            items(&mut cache),
            [5, 4, 3, 2],
            "Least-recently-used item evicted."
        );

        cache.insert(6);
        cache.insert(7);
        cache.insert(8);
        cache.insert(9);
        assert_eq!(
            items(&mut cache),
            [9, 8, 7, 6],
            "Least-recently-used item evicted."
        );
    }

    #[test]
    fn lookup() {
        let mut cache = TestCache::default();
        cache.insert(1);
        cache.insert(2);
        cache.insert(3);
        cache.insert(4);

        let result = cache.lookup(|x| if *x == 5 { Some(()) } else { None });
        assert_eq!(result, None, "Cache miss.");
        assert_eq!(items(&mut cache), [4, 3, 2, 1], "Order not changed.");

        // Cache hit
        let result = cache.lookup(|x| if *x == 3 { Some(*x * 2) } else { None });
        assert_eq!(result, Some(6), "Cache hit.");
        assert_eq!(
            items(&mut cache),
            [3, 4, 2, 1],
            "Matching item moved to front."
        );
    }

    #[test]
    fn clear() {
        let mut cache = TestCache::default();
        cache.insert(1);
        cache.clear();
        assert_eq!(items(&mut cache), [], "all items evicted");

        cache.insert(1);
        cache.insert(2);
        cache.insert(3);
        cache.insert(4);
        assert_eq!(items(&mut cache), [4, 3, 2, 1]);
        cache.clear();
        assert_eq!(items(&mut cache), [], "all items evicted again");
    }

    #[test]
    fn touch() {
        let mut cache = TestCache::default();

        cache.insert(0);
        cache.insert(1);
        cache.insert(2);
        cache.insert(3);

        cache.touch(|x| *x == 5);

        assert_eq!(items(&mut cache), [3, 2, 1, 0], "Nothing is touched.");

        cache.touch(|x| *x == 1);

        assert_eq!(
            items(&mut cache),
            [1, 3, 2, 0],
            "Touched item is moved to front."
        );
    }

    #[test]
    fn find() {
        let mut cache = TestCache::default();

        cache.insert(0);
        cache.insert(1);
        cache.insert(2);
        cache.insert(3);

        let result = cache.find(|x| *x == 5).copied();

        assert_eq!(result, None);
        assert_eq!(items(&mut cache), [3, 2, 1, 0], "Nothing is touched.");

        let result = cache.find(|x| *x == 1).copied();

        assert_eq!(result, Some(1));
        assert_eq!(
            items(&mut cache),
            [1, 3, 2, 0],
            "Retrieved item is moved to front."
        );
    }

    #[test]
    fn front() {
        let mut cache = TestCache::default();

        assert_eq!(cache.front(), None, "Nothing is in the front.");

        cache.insert(0);
        cache.insert(1);

        assert_eq!(
            cache.front(),
            Some(&1),
            "The last inserted item should be in the front."
        );

        cache.touch(|x| *x == 0);

        assert_eq!(
            cache.front(),
            Some(&0),
            "Touched item should be in the front."
        );
    }

    #[test]
    fn get() {
        let mut cache = TestCache::default();

        assert_eq!(cache.get(0), None, "Nothing at index 0.");

        cache.insert(42);
        cache.insert(64);

        assert_eq!(
            cache.get(0),
            Some(&64),
            "The last inserted item should be at index 0."
        );

        assert_eq!(
            cache.get(1),
            Some(&42),
            "The first inserted item should be at index 1."
        );

        cache.touch(|x| *x == 42);

        assert_eq!(
            cache.get(0),
            Some(&42),
            "The last touched item should be at index 0."
        );

        assert_eq!(
            cache.get(1),
            Some(&64),
            "The previously front item should be at index 1."
        );
    }
}
