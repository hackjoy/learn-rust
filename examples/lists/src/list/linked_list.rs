use std::collections::LinkedList;

pub fn create_linked_list() -> LinkedList<i32> {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);
    ll
}

#[cfg(test)]
mod test {
    use crate::list::linked_list::create_linked_list;

    #[test]
    fn basics() {
        let list = create_linked_list();
        for el in list {
            println!("{}", el)
        }
    }
}

// can run cargo test with `cargo test -- --nocapture` to see println! output
