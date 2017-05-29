use alloc::boxed::Box;

use core::fmt;
use core::ptr::Shared;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::hash::{Hash, Hasher};
use core::iter::{FromIterator, FusedIterator};

use collection_traits::*;


struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    #[inline(always)]
    fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element: element,
        }
    }
    #[inline(always)]
    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}


pub struct LinkedList<T> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

impl<T> LinkedList<T> {
    #[inline(always)]
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Shared::new(Box::into_raw(node)));

            match self.head {
                None => self.tail = node,
                Some(head) => (*(head.as_ptr() as *mut Node<T>)).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr() as *mut Node<T>);
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*(head.as_ptr() as *mut Node<T>)).prev = None,
            }

            self.len -= 1;
            node
        })
    }
    #[inline]
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Shared::new(Box::into_raw(node)));

            match self.tail {
                None => self.head = node,
                Some(tail) => (*(tail.as_ptr() as *mut Node<T>)).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }
    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr() as *mut Node<T>);
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(tail) => (*(tail.as_ptr() as *mut Node<T>)).next = None,
            }

            self.len -= 1;
            node
        })
    }

    #[inline]
    unsafe fn find_from_back(&self, index: usize) -> Option<Shared<Node<T>>> {
        let mut node = self.head;
        let mut i = 0;

        while i < index {
            node.map(|n| node = (*n.as_ptr()).next);
            i += 1;
        }
        node
    }
    #[inline]
    unsafe fn find_from_front(&self, index: usize) -> Option<Shared<Node<T>>> {
        let mut node = self.tail;
        let mut i = self.len - 1;

        while index < i {
            node.map(|n| node = (*n.as_ptr()).prev);
            i -= 1;
        }
        node
    }
    #[inline]
    unsafe fn find_from_back_mut(&mut self, index: usize) -> Option<Shared<Node<T>>> {
        let mut node = self.head;
        let mut i = 0;

        while i < index {
            node.map(|n| node = (*n.as_ptr()).next);
            i += 1;
        }
        node
    }
    #[inline]
    unsafe fn find_from_front_mut(&mut self, index: usize) -> Option<Shared<Node<T>>> {
        let mut node = self.tail;
        let mut i = self.len - 1;

        while index < i {
            node.map(|n| node = (*n.as_ptr()).prev);
            i -= 1;
        }
        node
    }

    #[inline(always)]
    unsafe fn find_node(&self, index: usize) -> Option<Shared<Node<T>>> {
        if (self.len - index) > index {
            self.find_from_back(index)
        } else {
            self.find_from_front(index)
        }
    }
    #[inline(always)]
    unsafe fn find_node_mut(&mut self, index: usize) -> Option<Shared<Node<T>>> {
        if (self.len - index) > index {
            self.find_from_back_mut(index)
        } else {
            self.find_from_front_mut(index)
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
    #[inline(always)]
    pub fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("index out of bounds")
    }
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        assert!(index < self.len);
        unsafe {
            self.find_node(index).map(|node| &(*node.as_ptr()).element)
        }
    }
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        assert!(index < self.len);
        unsafe {
            self.find_node_mut(index).map(|node| &mut (*(node.as_ptr() as *mut Node<T>)).element)
        }
    }
}

impl<T> Default for LinkedList<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    #[inline]
    fn drop(&mut self) {
        self.clear()
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_unchecked(index)
    }
}
impl<T> IndexMut<usize> for LinkedList<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_unchecked_mut(index)
    }
}

impl<T> Collection for LinkedList<T> {
    #[inline(always)]
    fn len(&self) -> usize { self.len }
    #[inline(always)]
    fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Insert<usize, T> for LinkedList<T> {
    type Output = ();

    #[inline]
    fn insert(&mut self, index: usize, element: T) -> Self::Output {
        let len = self.len;
        assert!(index <= len);

        if index == 0 {
            self.push_back(element)
        } else if index == len {
            self.push_front(element)
        } else {
            unsafe {
                let prev = self.find_node_mut(index - 1).expect("failed to find prev node");
                let next = (*prev.as_ptr()).next.expect("failed to find next node");

                let mut node = Box::new(Node::new(element));
                node.next = Some(next);
                node.prev = Some(prev);

                let node = Some(Shared::new(Box::into_raw(node)));
                (*(prev.as_ptr() as *mut Node<T>)).next = node;
                (*(next.as_ptr() as *mut Node<T>)).prev = node;

                self.len += 1;
            }
        }
    }
}

impl<T> Remove<usize> for LinkedList<T> {
    type Output = T;

    #[inline]
    fn remove(&mut self, index: usize) -> T {
        let len = self.len;
        assert!(index < len);

        if index == 0 {
            self.pop_back().unwrap()
        } else if index == len {
            self.pop_front().unwrap()
        } else {
            unsafe {
                let prev = self.find_node_mut(index - 1).expect("failed to find prev node");
                let node = (*prev.as_ptr()).next.expect("failed to find node");
                let next = (*node.as_ptr()).next.expect("failed to find next node");
                (*(prev.as_ptr() as *mut Node<T>)).next = Some(next);
                (*(next.as_ptr() as *mut Node<T>)).prev = Some(prev);
                self.len -= 1;
                Box::from_raw(node.as_ptr() as *mut Node<T>).into_element()
            }
        }
    }
}

impl<T> Deque<T> for LinkedList<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        self.push_front_node(Box::new(Node::new(element)));
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        self.push_back_node(Box::new(Node::new(element)));
    }
    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }
    #[inline(always)]
    fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).element })
    }
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).element })
    }
    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        self.head.map(|node| unsafe { &mut (*(node.as_ptr() as *mut Node<T>)).element })
    }
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.map(|node| unsafe { &mut (*(node.as_ptr() as *mut Node<T>)).element })
    }
}

impl<T> Stack<T> for LinkedList<T> {
    #[inline(always)]
    fn push(&mut self, element: T) { self.push_back(element) }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> { self.pop_back() }
    #[inline(always)]
    fn top(&self) -> Option<&T> { self.back() }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> Queue<T> for LinkedList<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) { self.push_back(element) }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> { self.pop_front() }
    #[inline(always)]
    fn peek(&self) -> Option<&T> { self.front() }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> FromIterator<T> for LinkedList<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline(always)]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            list: self,
        }
    }
}
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for element in self {
            element.hash(state);
        }
    }
}


pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}


pub struct Iter<'a, T: 'a> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

unsafe impl<'a, T: 'a + Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: 'a + Sync> Sync for Iter<'a, T> {}

impl<'a, T> Clone for Iter<'a, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &node.element
            })
        }
    }
}

impl<'a, T: 'a> Iterable<'a, &'a T> for LinkedList<T> {
    type Iter = Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}


pub struct IterMut<'a, T: 'a> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

unsafe impl<'a, T: 'a + Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: 'a + Sync> Sync for IterMut<'a, T> {}

impl<'a, T> Clone for IterMut<'a, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &mut *(node.as_ptr() as *mut Node<T>);
                self.len -= 1;
                self.head = node.next;
                &mut node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                let node = &mut *(node.as_ptr() as *mut Node<T>);
                self.len -= 1;
                self.tail = node.prev;
                &mut node.element
            })
        }
    }
}

impl<'a, T: 'a> IterableMut<'a, &'a mut T> for LinkedList<T> {
    type IterMut = IterMut<'a, T>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<'a, T: 'a> Seq<'a, T> for LinkedList<T> {}
impl<'a, T: 'a> SeqMut<'a, T> for LinkedList<T> {}
