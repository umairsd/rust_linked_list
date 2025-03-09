//

// V1
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// V2
// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

// V3
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
}
