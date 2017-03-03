extern crate linked_list;
extern crate collection_traits;


use linked_list::LinkedList;
use collection_traits::*;


const SIZE: usize = 32;


#[test]
fn test_linked_list() {
    let mut a = LinkedList::new();

    for i in 0..SIZE {
        a.push(i);
    }
    for i in 0..SIZE {
        assert_eq!(a.get(i).unwrap(), &i);
    }
    for i in 0..SIZE {
        *a.get_mut(i).unwrap() = SIZE - i;
    }
    while !a.is_empty() {
        a.pop();
    }

    assert!(a.is_empty());
}
