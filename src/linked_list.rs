use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait LinkedListNode<T: Clone + Debug> {
    fn create_node(
        content: Option<T>,
        next_node: Option<Rc<RefCell<Self>>>,
    ) -> Option<Rc<RefCell<Self>>>;
    fn set_node(&mut self, content: Option<T>);
    fn set_next_node(&mut self, node: Option<Rc<RefCell<Self>>>);
    fn get_next_node(&self) -> Option<Rc<RefCell<Self>>>;
}

// I think that there is some misuse of Rc::clone in the code,
// especially in the relation between `set_next_node` and `get_next_node`
pub mod single_linked_list {
    use super::LinkedListNode;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::rc::Rc;

    type SLLN<T> = Option<Rc<RefCell<SinglyLinkedListNode<T>>>>;

    #[derive(Clone, Debug)]
    pub struct SinglyLinkedListNode<T: Clone + Debug> {
        content: T,
        // It needs to be RefCell<Self> otherwise it could not be
        // changed into mut and use set_node method
        next: Option<Rc<RefCell<Self>>>,
    }

    impl<T: Clone + Debug> LinkedListNode<T> for SinglyLinkedListNode<T> {
        fn create_node(
            content: Option<T>,
            next_node: Option<Rc<RefCell<Self>>>,
        ) -> Option<Rc<RefCell<Self>>> {
            match content {
                Some(content) => Some(Rc::new(RefCell::new(SinglyLinkedListNode {
                    content,
                    // Whether it is necessary to move the ownership
                    // in create_node?
                    next: next_node,
                }))),
                None => None,
            }
        }

        fn set_node(&mut self, content: Option<T>) {
            match content {
                Some(content) => {
                    self.content = content;
                }
                None => {
                    panic!("Nothing to be set in node");
                }
            }
        }

        fn set_next_node(&mut self, node: Option<Rc<RefCell<Self>>>) {
            match node {
                Some(node) => {
                    self.next = Some(Rc::clone(&node));
                }
                None => {
                    self.next = None;
                }
            }
        }

        fn get_next_node(&self) -> Option<Rc<RefCell<Self>>> {
            match &self.next {
                Some(next_node) => Some(Rc::clone(&next_node)),
                None => None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct SinglyLinkedList<T: Clone + Debug> {
        head: SLLN<T>,
        len: u32,
    }

    impl<T: Clone + Debug> SinglyLinkedList<T> {
        pub fn new() -> Self {
            SinglyLinkedList { head: None, len: 0 }
        }

        pub fn get_head_node(&self) -> SLLN<T> {
            let r = self.head.clone().unwrap();
            return Some(r);
        }

        pub fn iter(&self) -> SinglyLinkedListIterator<T> {
            match &self.head {
                Some(_node) => {
                    SinglyLinkedListIterator::new(Some(Rc::clone(&(self.head.clone().unwrap()))))
                }
                None => SinglyLinkedListIterator::new(None),
            }
        }

        /// Use head insert because we do not know the tail
        pub fn head_insert_element(&mut self, ele: T) {
            let temp_head = self.head.clone();

            match temp_head {
                Some(_head) => {
                    self.head = SinglyLinkedListNode::create_node(Some(ele), self.head.clone());
                    self.len += 1;
                }
                None => {
                    self.head = SinglyLinkedListNode::create_node(Some(ele), None);
                }
            }
        }

        pub fn tail_insert_node(&mut self, node: SLLN<T>) {
            let mut temp_list_iterator = self.clone().iter();
            let mut temp_ele: SLLN<T> = None;

            while let Some(ele) = temp_list_iterator.next_node() {
                temp_ele = Some(ele);
            }
            match temp_ele {
                Some(temp_ele) => {
                    temp_ele.borrow_mut().set_next_node(node);
                }
                None => {
                    // this happend when self.head is None
                    self.head = Some(Rc::clone(&node.unwrap()));
                }
            }
        }

        /// It has to be implemented by tail insert, because `node` could be another list's node
        pub fn add_node(&mut self, node: SLLN<T>) {
            if let None = self.head {
                self.head = node;
                return;
            }
            match node {
                Some(node) => {
                    self.tail_insert_node(Some(node));
                }
                None => {
                    return;
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct SinglyLinkedListIterator<T: Clone + Debug> {
        pos: SLLN<T>,
    }

    impl<T: Clone + Debug> SinglyLinkedListIterator<T> {
        pub fn new(head: SLLN<T>) -> Self {
            match head {
                Some(head) => SinglyLinkedListIterator {
                    pos: Some(Rc::clone(&head)),
                },
                None => SinglyLinkedListIterator { pos: None },
            }
        }

        fn next_node(&mut self) -> SLLN<T> {
            match self.pos.clone() {
                Some(node_pos) => {
                    let res = Some(node_pos.clone());
                    self.pos = node_pos.borrow().get_next_node();
                    res
                }
                None => return None,
            }
        }
    }

    impl<T: Clone + Debug> Iterator for SinglyLinkedListIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            match &mut self.pos {
                Some(node_pos) => {
                    let ret = Some(node_pos.borrow().content.clone());
                    let pos_clone = self.pos.clone();
                    self.pos = pos_clone.unwrap().borrow().get_next_node();
                    ret
                }
                None => return None,
            }
        }
    }
}
