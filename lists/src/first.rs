// This is a impl of bad stack

// In functional programming, a List can be defined as
// List a = Empty | Elem a (List a)
// A List is either Empty or an Element followed by a List

// pub enum List {
//     Empty,
//     // Elem(i32, Box<List>),
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }
// struct Node {
//     elem: i32,
//     next: List,
// }
// pub enum List {
//     Empty,
//     More(Box<Node>),
// }
//
use std::mem;

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // TODO
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // TODO
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         self.head.drop();
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {}
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop();
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        let mut list = super::List::new();

        // empty list
        assert_eq!(list.pop(), None);

        // populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // normal remove
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push
        list.push(4);
        list.push(5);

        // normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
