#![feature(test)]


extern crate test;

extern crate linked_list;
extern crate collection_traits;


use test::Bencher;

use collection_traits::*;


const SIZE: usize = 32;


#[bench]
fn bench_linked_list(b: &mut Bencher) {
    use linked_list::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push(i);
        }
        a
    });
}
#[bench]
fn bench_std_linked_list(b: &mut Bencher) {
    use std::collections::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push_front(i);
        }
        a
    });
}


#[bench]
fn bench_linked_list_iter_mut(b: &mut Bencher) {
    use linked_list::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push(i);
        }
        for i in a.iter_mut() {
            *i = 0;
        }
        a
    });
}
#[bench]
fn bench_std_linked_list_iter_mut(b: &mut Bencher) {
    use std::collections::LinkedList;

    b.iter(|| {
        let mut a = LinkedList::new();

        for i in 0..SIZE {
            a.push_front(i);
        }
        for i in a.iter_mut() {
            *i = 0;
        }
        a
    });
}
