// An Ok Singly-Linked Stack
// make this stack less sucky:
// * Deinvent the wheel 
// * Make the list able to handle any element type
// * Add peeking 
// * Make the list iterable


// use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        // TODO
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // let result;
        // match mem::replace(&mut self.head, None) {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         // TODO
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        // 用map写闭包,多是一件美逝
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // unimplemented!()

    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
        }
    }
}

// * IntoIter - T
// * IterMut - &mut T
// * Iter - &T

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically 
        self.0.pop()
    }
}

// Within a function body you generally can't talk about lifetimes, and wouldn't want to anyway. The compiler has full information 
// and can infer all the constraints to find the minimum lifetimes.
// However at the type and API-level, the compiler doesn't have all the information. It requires you to 
// tell it about the relationship between different lifetimes so it can figure out what you're doing.

// Only add lifetimes in functions and type signatures
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// NO lifetime here, List does not have any associated lifetimes
impl<T> List<T> {
    // declare a fresh lifetime here for the *exact* borrow that creates the iter.
    // Now &self needs to be valid as long as the Iter is around.
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     // Iter { next: self.head.as_ref().map(|node| &*node) }
    //     // Iter { next: self.head.as_deref() }
    //     Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
    // }
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
    }
}

// We *do* have a lifetime here, cause Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // self.next = node.next.as_ref().map(|node| &*node);
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = super::List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });
        
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));

    }

    #[test]
    fn into_iter() {
        let mut list = super::List::new();
        list.push(1); 
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = super::List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = super::List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}