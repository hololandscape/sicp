/// Single linked list
use std::mem;

// the public struct can hide the implementation detail
pub struct SingleLinkedList {
    head: Link,
}


enum Link{
    Empty,
    More(Box<Node>),
}

struct Node{
    elem: i32,
    next: Link,
}


impl SingleLinkedList{
    // Self is an alias for SingleLinkedList
    // We implement associated function name new for single-linked-list
    pub fn new()-> Self{
        // for new function we need to return a new instance
        SingleLinkedList{
            // we refer to variants of an enum using :: the namespacing operator
            head:Link::Empty
        } // we need to return the variant, so there without the ;
    }

    // As we know the primary forms that self can take: self, &mut self and &self, push will change the linked list
    // so we need &mut
    // The push method which the signature's first parameter is self
    pub fn push(&mut self, elem: i32){
        let new_node=Box::new(Node{
            elem,
            // mem::replace function lets us steal a value out of a borrow by replacing it with another value
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head=Link::More(new_node);
    }
    ///
    /// In pop function, we trying to:
    /// * check if the list is empty, so we use enum Option<T>, it can either be Some(T) or None
    ///   * if it's empty, return None
    ///   * if it's not empty
    ///     * remove the head of the list
    ///     * remove its elem
    ///     * replace the list's head with its next
    ///     * return Some(elem), as the situation if need
    /// 
    /// so, we need to remove the head, and return the value of the head
    pub fn pop(&mut self)-> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty=>None,
            Link::More(node)=>{
                self.head=node.next;
                Some(node.elem)
            }
        }
    }
}

/// The drop method of single-linked-list. There's a question that do we need to worry about cleaning up our list?
/// As we all know the ownership and borrow mechanism, so we know the type will clean automatically after it goes out the scope,
/// this implement by the Rust compiler automatically did which mean add trait `drop` for the automatically. 
/// 
/// So, the complier will implements Drop for `List->Link->Box<Node> ->Node` automatically and tail recursive to clean the elements
/// one by one. And we know the recursive will stop at Box<Node>
/// https://rust-unofficial.github.io/too-many-lists/first-drop.html
/// 
/// As we know we can't drop the contents of the Box after deallocating, so we need to manually write the iterative drop

impl Drop for SingleLinkedList{
    fn drop(&mut self){
        let mut cur_link=mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node)=cur_link{
            cur_link=mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbound recursion occurs.
        }
    }
}

#[cfg(test)]
mod test_single_linkd_list{
    
    use super::*;

    #[test]
    fn basics() {
        let mut list=SingleLinkedList::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}