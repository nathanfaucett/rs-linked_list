#![feature(fused)]
#![feature(alloc)]
#![feature(shared)]
#![no_std]


extern crate alloc;

extern crate collection_traits;


mod linked_list;


pub use self::linked_list::LinkedList;
