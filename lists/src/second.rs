// An Ok Singly-Linked Stack
// make this stack less sucky:
// * Deinvent the wheel 
// * Make the list able to handle any element type
// * Add peeking 
// * Make the list iterable


// use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        // TODO
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
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
}

impl Drop for List {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
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