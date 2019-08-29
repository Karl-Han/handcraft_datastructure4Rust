pub mod linked_list {
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
                    self.next = Self::create_node(None, None);
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
            return Some(Rc::clone(&self.head.clone().unwrap()));
        }

        pub fn iter(&self) -> SinglyLinkedListIterator<T> {
            match &self.head {
                Some(_) => {
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
                    //self.head = th.clone();
                    //th.unwrap().borrow_mut().set_next_node(Some(head));
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
                    //println!(
                    //    "{:?}\n{:?}\n\n",
                    //    self.pos,
                    //    Rc::strong_count(&self.pos.clone().unwrap())
                    //);
                    self.pos = pos_clone.unwrap().borrow().get_next_node();
                    ret
                }
                None => return None,
            }
        }
    }
}

#[test]
fn single_linked_list_test() {
    // Default test
    assert_eq!(1, 1);
    use linked_list::*;

    let mut list: SinglyLinkedList<usize> = SinglyLinkedList::new();

    list.tail_insert_node(SinglyLinkedListNode::create_node(Some(3), None));
    list.add_node(SinglyLinkedListNode::create_node(Some(2), None));
    list.head_insert_element(1);

    let vec: Vec<usize> = list.iter().collect();
    assert_eq!(vec, vec![1, 3, 2]);

    let mut list1: SinglyLinkedList<usize> = SinglyLinkedList::new();
    list1.head_insert_element(5);
    list1.add_node(list.get_head_node());

    let vec: Vec<usize> = list1.iter().collect();
    assert_eq!(vec, vec![5, 1, 3, 2]);

    let mut list2: SinglyLinkedList<usize> = SinglyLinkedList::new();
    list2.tail_insert_node(SinglyLinkedListNode::create_node(Some(7), None));
    list2.head_insert_element(6);
    list2.add_node(list.get_head_node());

    let vec: Vec<usize> = list2.iter().collect();
    assert_eq!(vec, vec![6, 7, 1, 3, 2]);
}
