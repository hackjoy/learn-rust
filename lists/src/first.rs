use std::mem;

pub struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // Not best practice but.. mem::replace steals value from a mut borrow (&mut self) by replacing it with another valid value
            // The current scope push() then becomes owner of the old value (e.g self.head). The original owner still has a valid instance at that memory location
            // Below sets current self.head as value for next and replaces self.head with Link::Empty
            next: mem::replace(&mut self.head, Link::Empty),
        });
        // Then set self.head from Link::Empty to the new node (with the old value for new_node.next)
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Not best practice, match on the current value of self.head and replace self.head with Link::Empty
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// Manual drop implementation to avoid stack overflow when dropping Box<Node> recursively
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        use super::List;

        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
