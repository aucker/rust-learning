use std::ptr;

// The narrator:
// The raw pointers are basically C's pointers. So they have:
// no inherent aliasing rules; no lifetimes; can be null; can be misaligned; can be dangling;
// can point to uninitialized memory; can be casted to and from integers; can be cast to point to a defferent type;

// NARRATOR:
// NOT USED FOR PRODUCTION

pub struct List<T> {
    head: Link<T>,
    // tail: Link<T>,
    // tail: Option<&'a mut Node<T>>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        // List { head: None, tail: None }
        List { head: None, tail: ptr::null_mut() }
    }

    // pub fn push(&'a mut self, elem: T) {
    //     let new_tail = Box::new(Node {
    //         elem: elem,
    //         // the end of the node, the next is None
    //         next: None,
    //     });

    //     // swap the old tail to point to the new tail
    //     // let old_tail = mem::replace(&mut self.tail, Some(new_tail));

    //     // match old_tail {
    //     //     Some(mut old_tail) => {
    //     //         // if the old_tail exists, update it to point to the new tail
    //     //         old_tail.next = Some(new_tail);
    //     //     }
    //     //     None => {
    //     //         // otherwise, update the head to point to it
    //     //         self.head = Some(new_tail)
    //     //     }
    //     // }

    //     // put the box in the right place, and then grab a reference to its Node
    //     let new_tail = match self.tail.take() {
    //         Some(old_tail) => {
    //             // if the old tail existed, update it to the point to the new tail
    //             old_tail.next = Some(new_tail);
    //             old_tail.next.as_deref_mut()
    //         }
    //         None => {
    //             // otherwise, update the head to point to it
    //             self.head = Some(new_tail);
    //             self.head.as_deref_mut()
    //         }
    //     };
    //     self.tail = new_tail;
    // }
    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null checks for null, equivalent to checking for None
        if !self.tail.is_null() {
            // if the old tail existed, update it to point to the new tail
            // hello compiler, I know I am doing something dangerous and I Promise
            // to be a good programmer who never makes mistakes
            // HAHAHAHAHAHAHAHAHAHAHA
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // otherwise update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;

    }

    pub fn pop(&mut self) -> Option<T> {
        // Grab the list's current head
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // if we're out of `head`, make sure to set the tail to `None`
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // check empty list behave right
        assert_eq!(list.pop(), None);

        // populate lists
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // push some more to make sure nothing corrupted
        list.push(4);
        list.push(5);

        // check normal remaval
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        // check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}

// MARK:
// MIR: mid-level intermediate representation
// TL;DR: it interprets your program and notices if you break the rules at runtime and Do An Undefined Behaviour.
// This is necessary because Undefined Behaviour is generally a thing that happens at runtime. If the issue could 
// be found at compile time, the compiler would just nake it an error!