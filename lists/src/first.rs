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
