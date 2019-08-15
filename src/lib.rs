use std::rc::Rc;

pub trait LinkedListNode<T : Clone> {
    fn create_node<'a>(content : Option<T>) -> Option<Rc<Box<Self>>>;
    fn set_node(&mut self, content : Option<T>);
    fn set_next_node(&mut self, node : Option<Rc<Box<Self>>>);
    fn get_next_node(&self) -> Option<Rc<Box<Self>>>;
}


struct SinglyLinkedListNode<T : Clone> {
    content : T,
    next : Option<Rc<Box<Self>>>
}

impl<T : Clone> LinkedListNode<T> for SinglyLinkedListNode<T> {
    fn create_node<'a>(content : Option<T>) -> Option<Rc<Box<Self>>> {
        match content {
            Some(content) => {
                Some(Rc::new(Box::new(SinglyLinkedListNode{
                    content,
                    next : Self::create_node(None)
                })))
            }
            None => {
                None
            }
        }
    }

    fn set_node(&mut self, content : Option<T>){
        match content {
            Some(content) => {
                self.content = content;
            }
            None => {
                panic!("Nothing to be set in node");
            }
        }
    }

    fn set_next_node(&mut self, node : Option<Rc<Box<Self>>>){
        match node {
            Some(node) => {
                self.next = Some(Rc::clone(&node));
            }
            None => {
                self.next = Self::create_node(None);
            }
        }
    }

    fn get_next_node(&self) -> Option<Rc<Box<Self>>>{
        match &self.next {
            Some(next_node) => {
                Some(Rc::clone(&next_node))
            }
            None => {
                None
            }
        }
    }
}

struct SinglyLinkedList<T : Clone> {
    head : Option<Rc<Box<SinglyLinkedListNode<T>>>>,
    len : u32
}

impl<T : Clone> SinglyLinkedList<T>{
    pub fn new() -> Self{
        SinglyLinkedList {
            head : None,
            len : 0
        }
    }

    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        SinglyLinkedListIterator::new(Some(Rc::clone(&(self.head.clone().unwrap()))))
    }
    
    /// Use head insert because we do not know the tail
    pub fn head_insert_element(&mut self, ele : T){
        let temp_head = self.head.clone();

        match temp_head {
            Some(head) => {
                let &mut th = SinglyLinkedListNode::create_node(Some(ele));
                self.head = th.unwrap().set_next_node(Some(head));
                self.head = th;
                self.len += 1;
            }
            None => {
                self.head = SinglyLinkedListNode::create_node(Some(ele));
            }
        }
    }

    pub fn tail_insert_node(&self, node : Option<Rc<Box<SinglyLinkedListNode<T>>>>) {
        let mut temp_list_iterator = self.clone().iter();
        
        loop {
            while let Some(_ele) = temp_list_iterator.next() {
            }
        }
    }

    /// It has to be implemented by tail insert, because `node` could be another list's node
    pub fn add_node(&mut self, node : Option<Rc<Box<SinglyLinkedListNode<T>>>>){
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

struct SinglyLinkedListIterator<T : Clone> {
    pos : Option<Rc<Box<SinglyLinkedListNode<T>>>>
}

impl<T : Clone> SinglyLinkedListIterator<T> {
    pub fn new(head : Option<Rc<Box<SinglyLinkedListNode<T>>>>) -> Self{
        SinglyLinkedListIterator{pos : Some(Rc::clone(&head.unwrap()))}
    }
}

impl<T : Clone> Iterator for SinglyLinkedListIterator<T>{
    type Item = T;

    fn next(&mut self) -> Option<T>{
        match &self.pos {
            Some(node_pos) => {
                Some(node_pos.content.clone())
            }
            None => {
                return None
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn single_linked_list_test(){
        unimplemented!();
    }
}
