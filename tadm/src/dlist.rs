use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

pub struct DList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _marker: PhantomData<T>,
}

pub type Link<T> = NonNull<Node<T>>;

pub struct Node<T> {
    pub elem: MaybeUninit<T>,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem: MaybeUninit::new(elem),
            prev: NonNull::dangling(),
            next: NonNull::dangling(),
        }
    }

    fn new_dummy() -> Self {
        Node {
            elem: MaybeUninit::uninit(),
            prev: NonNull::dangling(),
            next: NonNull::dangling(),
        }
    }
}

impl<T> DList<T> {
    pub fn new() -> Self {
        unsafe {
            let dummy_head = NonNull::new(Box::into_raw(Box::new(Node::new_dummy()))).unwrap();
            let dummy_tail = NonNull::new(Box::into_raw(Box::new(Node::new_dummy()))).unwrap();

            (*dummy_head.as_ptr()).next = dummy_tail;
            (*dummy_head.as_ptr()).prev = dummy_tail;

            (*dummy_tail.as_ptr()).next = dummy_head;
            (*dummy_tail.as_ptr()).prev = dummy_head;

            DList {
                head: dummy_head,
                tail: dummy_tail,
                len: 0,
                _marker: PhantomData,
            }
        }
    }

    pub fn move_back_link(&mut self, link: Link<T>) {
        self.remove_link(link);
        self.push_back_link(link)
    }

    pub fn push_front_link(&mut self, link: Link<T>) {
        unsafe {
            (*link.as_ptr()).prev = self.head;
            (*link.as_ptr()).next = (*self.head.as_ptr()).next;
            (*(*self.head.as_ptr()).next.as_ptr()).prev = link;
            (*self.head.as_ptr()).next = link;

            self.len += 1;
        }
    }

    pub fn push_back_link(&mut self, link: Link<T>) {
        unsafe {
            (*link.as_ptr()).next = self.tail;
            (*link.as_ptr()).prev = (*self.tail.as_ptr()).prev;
            (*(*self.tail.as_ptr()).prev.as_ptr()).next = link;
            (*self.tail.as_ptr()).prev = link;

            self.len += 1;
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_link = NonNull::new(Box::into_raw(Box::new(Node::new(elem)))).unwrap();

        self.push_front_link(new_link);
    }

    pub fn push_back(&mut self, elem: T) {
        let new_link = NonNull::new(Box::into_raw(Box::new(Node::new(elem)))).unwrap();

        self.push_back_link(new_link);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if self.len == 0 {
                None
            } else {
                let front_link = (*self.head.as_ptr()).next;

                self.remove_link(front_link);

                let front_node = Box::from_raw(front_link.as_ptr());

                Some(front_node.elem.assume_init())
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            if self.len == 0 {
                None
            } else {
                let back_link = (*self.tail.as_ptr()).prev;

                self.remove_link(back_link);

                let back_node = Box::from_raw(back_link.as_ptr());

                Some(back_node.elem.assume_init())
            }
        }
    }

    fn remove_link(&mut self, link: Link<T>) {
        unsafe {
            (*(*link.as_ptr()).prev.as_ptr()).next = (*link.as_ptr()).next;
            (*(*link.as_ptr()).next.as_ptr()).prev = (*link.as_ptr()).prev;

            self.len -= 1;
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe {
            if self.len > 0 {
                (*(*self.head.as_ptr()).next.as_ptr())
                    .elem
                    .as_ptr()
                    .as_ref()
            } else {
                None
            }
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            if self.len > 0 {
                (*(*self.head.as_ptr()).next.as_ptr())
                    .elem
                    .as_mut_ptr()
                    .as_mut()
            } else {
                None
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for DList<T> {
    fn drop(&mut self) {
        while self.len() > 0 {
            self.pop_back();
        }

        unsafe {
            Box::from_raw(self.head.as_ptr());
            Box::from_raw(self.tail.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = DList::new();
        list.push_front('a');
        list.push_front('b');
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&'b'));
        assert_eq!(list.pop_front(), Some('b'));
        assert_eq!(list.front(), Some(&'a'));
        assert_eq!(list.pop_front(), Some('a'));
        assert_eq!(list.len(), 0);

        list.push_front('a');
        list.push_front('b');
        list.push_back('c');
        assert_eq!(list.len(), 3);

        list.push_back('d');
        list.push_back('e');
        assert_eq!(list.len(), 5);
        assert_eq!(list.pop_front(), Some('b'));
        assert_eq!(list.pop_back(), Some('e'));
        list.push_front('f');
        assert_eq!(list.pop_back(), Some('d'));
        assert_eq!(list.pop_back(), Some('c'));
        assert_eq!(list.pop_back(), Some('a'));
        assert_eq!(list.len(), 1)
    }
}
