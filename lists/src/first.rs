// 01: First implementation. A Bad Stack

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
        // v1: This doesn't work: "cannot move out of borrowed content"
        // let new_node = Node {
        //     elem: elem,
        //     next: self.head,
        // };

        // v2: This doesn't work either. Same error.
        // let new_node = Box::new(Node {
        //     elem: elem,
        //     next: self.head,
        // });
        // self.head = Link::More(new_node);

        // v3: Use std::mem::replace. Let's us steal a value out of a borrow
        // by replacing it with another value.
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    // V1:
    // pub fn pop(&mut self) -> Option<i32> {
    //     let result: Option<i32>;
    //     // Note: `mem::replace` returns the original value.
    //     let old_head = mem::replace(&mut self.head, Link::Empty);
    //     match old_head {
    //         Link::Empty => {
    //             result = None;
    //         }
    //         Link::More(node) => {
    //             result = Option::Some(node.elem);
    //             self.head = node.next;
    //         }
    //     };
    //     return result;
    // }

    // V2: Slightly more rusty.
    pub fn pop(&mut self) -> Option<i32> {
        // Note: `mem::replace` returns the original value.
        let old_head = mem::replace(&mut self.head, Link::Empty);
        match old_head {
            Link::Empty => Option::None,
            Link::More(node) => {
                self.head = node.next;
                Option::Some(node.elem)
            }
        }
    }
}
